mod constatnts;
mod entities;
mod history_fetcher;

use ::async_trait::async_trait;
use ::chrono::{DateTime, Duration, NaiveDateTime, Utc};
use ::crossbeam::channel::bounded;
use ::futures::stream::StreamExt;
use ::mongodb::{
  bson::{
    de::Result as BsonDeResult, doc, from_bson, to_bson, Array, Bson, Document,
  },
  error::Result as MongoResult,
  Collection,
};
use ::nats::Connection as NatsCon;
use ::rmp_serde::{from_slice as from_msgpack, to_vec as to_msgpack};
use ::serde_json::Value;
use ::slog::{o, warn, Logger};
use ::std::thread;
use ::tokio::sync::{broadcast, mpsc};
use ::tokio::task::block_in_place;
use ::types::{ret_on_err, SendableErrorResult};

use crate::entities::KlineCtrl;
use crate::traits::Exchange;
use ::config::{CHAN_BUF_SIZE, NUM_CONC_TASKS, NUM_OBJECTS_TO_FETCH};
use ::rand::{thread_rng, Rng};
use ::rpc::entities::SymbolInfo;
use ::rpc::historical::HistChartProg;

use self::entities::{
  ExchangeInfo, HistFetcherParam, Kline, KlineResultsWithSymbol, Symbol,
};
use self::history_fetcher::HistoryFetcher;
use super::errors::{DeterminationFailed, EmptyError, StatusFailure};

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
  fn gen_rand_range(&self, min: usize, max: usize) -> usize {
    return thread_rng().gen_range(min, max);
  }
  fn spawn_recorder(
    &self,
    mut stop: broadcast::Receiver<()>,
    value_ch: mpsc::Receiver<SendableErrorResult<KlineResultsWithSymbol>>,
    prog_ch: mpsc::Sender<SendableErrorResult<HistChartProg>>,
  ) {
    let mut value_ch = value_ch;
    let mut prog_ch = prog_ch;
    let me = self.clone();
    thread::spawn(move || {
      ::tokio::spawn(async move {
        while let Err(_) = stop.try_recv() {
          match value_ch.recv().await {
            None => break,
            Some(kline_reuslt) => match kline_reuslt {
              Err(err) => {
                let _ = prog_ch.send(Err(err));
                continue;
              }
              Ok(ok) => {
                let raw_klines = ok.klines;
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
                  cur_object_num: 1,
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
    let (stop_send, stop_recv) = bounded(0);
    let fetchers = (0..NUM_CONC_TASKS)
      .map(|index| {
        return HistoryFetcher::new(
          None,
          self
            .logger
            .new(o!("scope" => format!("History Fetcher [{}]", index))),
        );
      })
      .filter_map(|item| item.ok());
    let mut senders = vec![];
    let mut recvers = vec![];
    for fetcher in fetchers {
      let (param, res) = fetcher.spawn(stop_recv.clone());
      senders.push(param);
      recvers.push(res);
    }
    for recv_ch in recvers {
      self.spawn_recorder(stop_send.subscribe(), recv_ch, res_send.clone());
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
    let ctrl_subsc = ret_on_err!(self.broker.subscribe("binance.kline.ctrl"));
    let req_thread_logger = self.logger.new(o!("scope" => "Request Thread"));
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
          let mut entire_data_len = (end_at.clone() - start_at).num_minutes();
          let entire_data_len_rem =
            entire_data_len % NUM_OBJECTS_TO_FETCH as i64;
          entire_data_len /= 1000;
          if entire_data_len_rem > 0 {
            entire_data_len += 1;
          }
          let mut sec_end_date = end_at.clone();
          while sec_end_date > start_at {
            match ctrl_subsc.try_next() {
              Some(msg) => {
                match from_msgpack::<KlineCtrl>(&msg.data[..]) {
                  Err(err) => {
                    warn!(
                      req_thread_logger,
                      "Received Control Message, but failed to parse it: {}",
                      err
                    );
                  }
                  Ok(v) => match (v) {
                    KlineCtrl::Stop => {
                      let _ = stop_send.send(());
                      return;
                    }
                  },
                };
              }
              None => {}
            }
            let mut sec_start_date =
              sec_end_date - Duration::minutes(NUM_OBJECTS_TO_FETCH as i64);
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
