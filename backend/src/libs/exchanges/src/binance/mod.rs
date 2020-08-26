mod entities;

use ::async_trait::async_trait;
use ::chrono::{DateTime, Duration, NaiveDateTime, Utc};
use ::futures::stream::StreamExt;
use ::mongodb::{
  bson::{
    de::Result as BsonDeResult, doc, from_bson, to_bson, Array, Bson, Document,
  },
  error::Result as MongoResult,
  Collection,
};
use ::nats::Connection as NatsCon;
use ::rmp_serde::to_vec as to_msgpack;
use ::serde_json::Value;
use ::serde_qs::to_string;
use ::slog::{warn, Logger};
use ::std::thread;
use ::tokio::sync::mpsc;
use ::tokio::task::block_in_place;
use ::types::{ret_on_err, ParseURLResult, SendableErrorResult};

use crate::entities::KlineCtrl;
use crate::traits::Exchange;
use ::config::{CHAN_BUF_SIZE, DEFAULT_RECONNECT_INTERVAL, NUM_CONC_TASKS};
use ::rand::{thread_rng, Rng};
use ::rpc::entities::SymbolInfo;
use ::rpc::historical::HistChartProg;

use self::entities::{
  ExchangeInfo, HistFetcherParam, HistQuery, Kline, KlineResults,
  KlineResultsWithSymbol, Symbol,
};
use super::errors::{
  DeterminationFailed, EmptyError, MaximumAttemptExceeded, NumObjectError,
  StatusFailure,
};

type BinancePayload = Vec<Vec<Value>>;

#[derive(Debug, Clone)]
pub struct Binance {
  hist_col: Collection,
  syminfo_col: Collection,
  logger: Logger,
  broker: NatsCon,
}

impl Binance {
  pub fn new(
    logger: Logger,
    history_collection: Collection,
    symbol_info_collection: Collection,
    broker: NatsCon,
  ) -> Self {
    return Self {
      hist_col: history_collection,
      syminfo_col: symbol_info_collection,
      logger,
      broker,
    };
  }
  pub fn get_ws_endpoint(&self) -> ParseURLResult {
    return "wss://stream.binance.com:9443".parse();
  }
  pub fn get_rest_endpoint(&self) -> ParseURLResult {
    return "https://api.binance.com".parse();
  }
  async fn get_hist(
    &self,
    pair: String,
    num_symbols: i64,
    entire_data_len: i64,
    start_at: DateTime<Utc>,
    end_at: Option<DateTime<Utc>>,
  ) -> SendableErrorResult<KlineResultsWithSymbol> {
    let limit = match end_at {
      Some(end_at) => Some((end_at - start_at).num_minutes()),
      None => None,
    };
    let url = ret_on_err!(self.get_rest_endpoint());
    let mut url = ret_on_err!(url.join("/api/v3/klines"));
    let param = HistQuery {
      symbol: pair.clone(),
      interval: "1m".into(),
      start_time: format!("{}", start_at.timestamp()),
      end_time: match end_at {
        Some(end_at) => Some(format!("{}", end_at.timestamp())),
        None => None,
      },
      limit: format!("{}", limit.unwrap_or(1)),
    };
    let param = ret_on_err!(to_string(&param));
    url.set_query(Some(&param));
    let mut c: i8 = 0;
    while c < 20 {
      let resp = ret_on_err!(::reqwest::get(url.clone()).await);
      let rest_status = resp.status();
      if rest_status.is_success() {
        let values = ret_on_err!(resp.json::<BinancePayload>().await);
        let ret: KlineResults = values
          .iter()
          .map(|item| Ok(Kline::new(pair.clone(), item)?))
          .collect();
        return Ok(KlineResultsWithSymbol {
          symbol: pair,
          num_symbols,
          entire_data_len,
          klines: ret,
        });
      } else if rest_status == ::reqwest::StatusCode::IM_A_TEAPOT
        || rest_status == ::reqwest::StatusCode::TOO_MANY_REQUESTS
      {
        let retry_secs: i64 = match resp.headers().get("retry-after") {
          Some(t) => i64::from_str_radix(
            t.to_str()
              .unwrap_or(&DEFAULT_RECONNECT_INTERVAL.to_string()),
            10,
          )
          .unwrap_or(DEFAULT_RECONNECT_INTERVAL),
          None => DEFAULT_RECONNECT_INTERVAL,
        };
        let retry_secs = Duration::seconds(retry_secs);
        ::async_std::task::sleep(ret_on_err!(retry_secs.to_std())).await;
        warn!(
          self.logger,
          "Got code {}. Waiting for {} seconds...",
          rest_status.as_u16(),
          retry_secs.num_seconds(),
        );
      } else {
        let text = ret_on_err!(resp.text().await);
        warn!(
          self.logger, "Got code {}.",
          rest_status.as_u16(); "body" => text,
        );
      }
      c += 1;
    }
    return Err(Box::new(MaximumAttemptExceeded::default()));
  }

  fn spawn_history_fetcher(
    &self,
  ) -> (
    mpsc::Sender<HistFetcherParam>,
    mpsc::Receiver<SendableErrorResult<KlineResultsWithSymbol>>,
  ) {
    let (param_send, mut param_rec) =
      mpsc::channel::<HistFetcherParam>(CHAN_BUF_SIZE);
    let (mut prog_send, prog_rec) = mpsc::channel::<
      SendableErrorResult<KlineResultsWithSymbol>,
    >(CHAN_BUF_SIZE);
    let me = self.clone();
    thread::spawn(move || {
      ::tokio::spawn(async move {
        loop {
          let param_option = param_rec.recv().await;
          match param_option {
            Some(param) => {
              let num_obj = (param.end_time - param.start_time).num_minutes();
              if num_obj > 1000 {
                let _ = prog_send
                  .send(Err(Box::new(NumObjectError {
                    field: String::from("Duration between start and end date"),
                    num_object: 1000,
                  })))
                  .await;
                continue;
              }
              let _ = prog_send
                .send(
                  me.get_hist(
                    param.symbol.clone(),
                    param.num_symbols,
                    param.entire_data_len,
                    param.start_time,
                    Some(param.end_time),
                  )
                  .await,
                )
                .await;
            }
            None => break,
          }
        }
      });
    });
    return (param_send, prog_rec);
  }
  fn gen_rand_range(&self, min: usize, max: usize) -> usize {
    return thread_rng().gen_range(min, max);
  }
  fn spawn_recorder(
    &self,
    value_ch: mpsc::Receiver<SendableErrorResult<KlineResultsWithSymbol>>,
    prog_ch: mpsc::Sender<SendableErrorResult<HistChartProg>>,
  ) {
    let mut value_ch = value_ch;
    let mut prog_ch = prog_ch;
    let me = self.clone();
    thread::spawn(move || {
      ::tokio::spawn(async move {
        loop {
          match value_ch.recv().await {
            None => break,
            Some(kline_reuslt) => match kline_reuslt {
              Err(err) => {
                let _ = prog_ch.send(Err(err));
                continue;
              }
              Ok(ok) => {
                let raw_klines = ok.klines;
                let raw_klines_len = raw_klines.len();
                let empty = Array::new();
                let succeeded_klines: Vec<Kline> = raw_klines
                  .into_iter()
                  .filter_map(|item| item.ok())
                  .map(|item| item.clone())
                  .collect();
                let klines: Vec<Document> = to_bson(&succeeded_klines)
                  .unwrap_or(Bson::Array(Array::new()))
                  .as_array()
                  .unwrap_or(&empty)
                  .into_iter()
                  .filter_map(|item| item.as_document())
                  .map(|item| item.clone())
                  .collect();
                let db_insert =
                  me.hist_col.insert_many(klines.into_iter(), None);
                let _ = prog_ch.send(Ok(HistChartProg {
                  symbol: ok.symbol,
                  num_symbols: ok.num_symbols,
                  cur_symbol_num: 1,
                  num_objects: ok.entire_data_len,
                  cur_object_num: raw_klines_len as i64,
                }));
                let _ = db_insert.await;
              }
            },
          }
        }
      });
    });
  }

  async fn get_first_trade_date(
    &self,
    symbol: String,
  ) -> SendableErrorResult<DateTime<Utc>> {
    let start = self
      .get_hist(
        symbol.clone(),
        1,
        1,
        DateTime::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
        None,
      )
      .await?;
    let mut start = start.klines;
    start.retain(|item| item.is_ok());
    if start.len() != 1 {
      return Err(Box::new(DeterminationFailed::<()> {
        field: String::from("Start Date"),
        additional_data: None,
      }));
    }
    return Ok(start[0].as_ref().unwrap().open_time.into());
  }
}

#[async_trait]
impl Exchange for Binance {
  async fn refresh_historical(
    &self,
    symbol: Vec<String>,
  ) -> SendableErrorResult<mpsc::Receiver<SendableErrorResult<HistChartProg>>>
  {
    if symbol.len() < 1 {
      return Err(Box::new(EmptyError {
        field: String::from("symbol"),
      }));
    }
    let (res_send, res_recv) =
      mpsc::channel::<SendableErrorResult<HistChartProg>>(CHAN_BUF_SIZE);
    let mut senders = vec![];
    let mut recvers = vec![];
    for _ in 0..NUM_CONC_TASKS {
      let (param, res) = self.spawn_history_fetcher();
      senders.push(param);
      recvers.push(res);
    }
    for recv_ch in recvers {
      self.spawn_recorder(recv_ch, res_send.clone());
    }
    let mut query: Option<Document> = None;
    if symbol[0] != "all" {
      query = Some(doc! { "symbol": doc! { "$in": symbol } });
    }
    let symbols = self.get_symbols(query).await?;
    let symbols_len = symbols.len();
    let end_at = Utc::now();
    let me = self.clone();
    let mut res_send_in_thread = res_send.clone();
    thread::spawn(move || {
      ::tokio::spawn(async move {
        for symbol in symbols {
          let start_at =
            match me.get_first_trade_date(symbol.symbol.clone()).await {
              Err(e) => {
                let _ = res_send_in_thread.send(Err(e));
                break;
              }
              Ok(v) => v,
            };
          let entire_data_len = (end_at.clone() - start_at).num_minutes();
          let mut sec_end_date = end_at.clone();
          while sec_end_date > start_at {
            let mut sec_start_date = sec_end_date - Duration::minutes(1000);
            if sec_start_date < start_at {
              sec_start_date = start_at;
            }
            let index = me.gen_rand_range(0, senders.len());
            let sender = &mut senders[index];
            let _ = sender.send(HistFetcherParam {
              symbol: symbol.symbol.clone(),
              num_symbols: symbols_len as i64,
              entire_data_len,
              start_time: sec_start_date.clone(),
              end_time: sec_end_date,
            });
            sec_end_date = sec_start_date.clone();
          }
        }
      });
    });
    return Ok(res_recv);
  }

  async fn get_symbols(
    &self,
    filter: impl Into<Option<Document>> + Send + 'async_trait,
  ) -> SendableErrorResult<Vec<SymbolInfo>> {
    let cur = ret_on_err!(self.syminfo_col.find(filter, None).await);
    let mut docs: Vec<MongoResult<Document>> = cur.collect().await;
    docs.retain(|doc| doc.is_ok());
    let mut symbols: Vec<BsonDeResult<Symbol>> = docs
      .iter()
      .map(|doc_res| {
        let doc = doc_res.clone().unwrap();
        let item: BsonDeResult<Symbol> = from_bson(Bson::Document(doc));
        return item;
      })
      .collect();
    symbols.retain(|item| item.is_ok());
    let ret = symbols
      .into_iter()
      .map(|item| item.unwrap().as_symbol_info())
      .collect();
    return Ok(ret);
  }

  async fn refresh_symbols(self) -> SendableErrorResult<()> {
    let mut url: url::Url = ret_on_err!(self.get_rest_endpoint());
    url = ret_on_err!(url.join("/api/v3/exchangeInfo"));
    let resp = ret_on_err!(reqwest::get(url.clone()).await);
    let resp_status = resp.status();
    if resp_status.is_success() {
      let info: ExchangeInfo = ret_on_err!(resp.json().await);
      ret_on_err!(self.syminfo_col.delete_many(doc! {}, None).await);
      let empty = Array::new();
      let serialized: Vec<Document> = ret_on_err!(to_bson(&info.symbols))
        .as_array()
        .unwrap_or(&empty)
        .into_iter()
        .filter_map(|item| item.as_document())
        .map(|item| item.clone())
        .collect();
      ret_on_err!(
        self
          .syminfo_col
          .insert_many(serialized.into_iter(), None)
          .await
      );
      return Ok(());
    } else {
      return Err(Box::new(StatusFailure {
        url: url.clone(),
        code: resp_status.as_u16(),
        text: ret_on_err!(resp.text().await),
      }));
    }
  }

  async fn stop(self) -> SendableErrorResult<()> {
    let msg = ret_on_err!(to_msgpack(&KlineCtrl::Stop));
    ret_on_err!(block_in_place(move || {
      self.broker.publish("binance.kline.ctrl", &msg[..])
    }));
    return Ok(());
  }
}
