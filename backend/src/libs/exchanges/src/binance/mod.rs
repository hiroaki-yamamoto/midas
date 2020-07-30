mod entities;
mod errors;

use ::async_trait::async_trait;
use ::chrono::{DateTime, Duration, Utc};
use ::futures::stream::StreamExt;
use ::mongodb::{
  bson::{de::Result as BsonDeResult, doc, from_bson, to_bson, Bson, Document},
  error::Result as MongoResult,
  Collection,
};
use ::serde_json::Value;
use ::serde_qs::to_string;
use ::slog::{warn, Logger};
use ::std::error::Error;
use ::std::thread;
use ::tokio::sync::{mpsc, oneshot};
use ::types::{ret_on_err, ParseURLResult, SendableErrorResult};

use ::rpc::entities::SymbolInfo;
use ::rpc::historical::HistChartProg;

use crate::casting::{cast_datetime, cast_f64, cast_i64};
use crate::traits::Exchange;

use self::entities::{ExchangeInfo, HistFetcherParam, HistQuery, Kline, Symbol};
use self::errors::{MaximumAttemptExceeded, StatusFailure};

type BinancePayload = Vec<Vec<Value>>;
pub type BinaceKlineResults = Vec<Result<Kline, Box<dyn Error + Send>>>;

const DEFAULT_RECONNECT_INTERVAL: i64 = 30;
const CHAN_BUF_SIZE: usize = 1024;
const NUM_CONC_TASKS: u8 = 6;

#[derive(Debug, Clone)]
pub struct Binance {
  hist_col: Collection,
  syminfo_col: Collection,
  logger: Logger,
}

impl Binance {
  pub fn new(
    logger: Logger,
    histry_collection: Collection,
    symbol_info_collection: Collection,
  ) -> Self {
    return Self {
      hist_col: histry_collection,
      syminfo_col: symbol_info_collection,
      logger,
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
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
  ) -> SendableErrorResult<BinaceKlineResults> {
    let limit = (end_at - start_at).num_minutes();
    let url = ret_on_err!(self.get_rest_endpoint());
    let mut url = ret_on_err!(url.join("/api/v3/klines"));
    let param = HistQuery {
      symbol: pair,
      interval: "1m".into(),
      start_time: format!("{}", start_at.timestamp()),
      end_time: format!("{}", end_at.timestamp()),
      limit: format!("{}", limit),
    };
    let param = ret_on_err!(to_string(&param));
    url.set_query(Some(&param));
    let mut c: i8 = 0;
    while c < 20 {
      let resp = ret_on_err!(::reqwest::get(url.clone()).await);
      let rest_status = resp.status();
      if rest_status.is_success() {
        let values = ret_on_err!(resp.json::<BinancePayload>().await);
        let ret: BinaceKlineResults = values
          .iter()
          .map(|item| {
            return Ok(Kline {
              open_time: cast_datetime("open_time", item[0].clone())?,
              open_price: cast_f64("open_price", item[1].clone())?,
              high_price: cast_f64("high_price", item[2].clone())?,
              low_price: cast_f64("low_price", item[3].clone())?,
              close_price: cast_f64("close_price", item[4].clone())?,
              volume: cast_f64("volume", item[5].clone())?,
              close_time: cast_datetime("close_time", item[6].clone())?,
              quote_volume: cast_f64("quote_volume", item[7].clone())?,
              num_trades: cast_i64("num_trades", item[8].clone())?,
              taker_buy_base_volume: cast_f64("taker_buy_base_volume", item[9].clone())?,
              taker_buy_quote_volume: cast_f64("taker_buy_quote_volume", item[10].clone())?,
            });
          })
          .collect();
        return Ok(ret);
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
    self,
  ) -> (
    mpsc::Sender<HistFetcherParam>,
    mpsc::Receiver<Result<BinaceKlineResults, Box<dyn Error + Send>>>,
  ) {
    let (param_send, mut param_rec) = mpsc::channel::<HistFetcherParam>(CHAN_BUF_SIZE);
    let (mut prog_send, prog_rec) = mpsc::channel(CHAN_BUF_SIZE);
    thread::spawn(move || {
      ::tokio::spawn(async move {
        loop {
          let param_option = param_rec.recv().await;
          match param_option {
            Some(param) => {
              let num_obj = (param.end_time - param.start_time).num_minutes();
              let mut num_loop = num_obj / 1000;
              if num_obj % 1000 > 0 {
                num_loop += 1;
              }
              for i in 0..num_loop {
                let start_time = param.start_time + Duration::minutes(1000 * i);
                let mut end_time = start_time + Duration::minutes(1000);
                if end_time > param.end_time {
                  end_time = param.end_time;
                }
                let _ = prog_send
                  .send(
                    self
                      .clone()
                      .get_hist(param.symbol.clone(), start_time, end_time)
                      .await,
                  )
                  .await;
              }
            }
            None => break,
          }
        }
      });
    });
    return (param_send, prog_rec);
  }
}

#[async_trait]
impl Exchange for Binance {
  async fn refresh_historical(
    self,
    symbol: Vec<String>,
  ) -> (oneshot::Sender<()>, mpsc::Receiver<HistChartProg>) {
    let (stop_send, stop_recv) = oneshot::channel::<()>();
    let (res_send, res_recv) = mpsc::channel::<HistChartProg>(CHAN_BUF_SIZE);
    let mut senders = vec![];
    let mut recvers = vec![];
    for _ in 0..NUM_CONC_TASKS {
      let (param, res) = self.clone().spawn_history_fetcher();
      senders.push(param);
      recvers.push(res);
    }
    if symbol.len() == 1 && symbol[0] == "all" {}
    return (stop_send, res_recv);
  }

  async fn get_symbols(&self) -> SendableErrorResult<Vec<SymbolInfo>> {
    let cur = ret_on_err!(self.syminfo_col.find(doc! {}, None).await);
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
      .map(|item| {
        let sym = item.unwrap();
        return SymbolInfo {
          symbol: sym.symbol,
          base: sym.base_asset,
          quote: sym.quote_asset,
        };
      })
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
      let serialized = ret_on_err!(to_bson(&info.symbols));
      let empty: Vec<::mongodb::bson::Bson> = vec![];
      let mut docs: Vec<Option<&Document>> = serialized
        .as_array()
        .unwrap_or(&empty)
        .into_iter()
        .map(|item| item.as_document())
        .collect();
      docs.retain(|item| item.is_some());
      ret_on_err!(
        self
          .syminfo_col
          .insert_many(docs.into_iter().map(|doc| { doc.unwrap().clone() }), None)
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
}
