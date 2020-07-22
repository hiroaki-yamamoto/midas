use ::chrono::{DateTime, Utc, Duration};
use ::serde::Serialize;
use ::serde_json::Value;
use ::serde_qs::to_string;
use ::std::error::Error;
use ::std::fmt::{Display, Formatter, Result as FormatResult};
use ::types::{ParseURLResult, GenericResult};
use ::async_trait::async_trait;
use ::slog::{Logger, warn};
use ::tokio::sync::mpsc;

use ::rpc::historical::HistChartProg;
use ::rpc::entities::SymbolInfo;

use crate::traits::Exchange;
use crate::casting::{cast_datetime, cast_f64, cast_i64};

type BinancePayload = Vec<Vec<Value>>;
type BinaceKlineResults = Vec<Result<Kline, Box<dyn Error>>>;

const DEFAULT_RECONNECT_INTERVAL: i64 = 30;
const CHAN_BUF_SIZE: usize = 1024;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HistQuery {
  pub symbol: String,
  pub interval: String,
  pub start_time: String,
  pub end_time: String,
  pub limit: String,
}

pub struct HistFetcherParam {
  pub symbol: String,
  pub start_time: DateTime<Utc>,
  pub end_time: DateTime<Utc>,
}

struct Kline {
  pub open_time: DateTime<Utc>,
  pub open_price: f64,
  pub high_price: f64,
  pub low_price: f64,
  pub close_price: f64,
  pub volume: f64,
  pub close_time: DateTime<Utc>,
  pub quote_volume: f64,
  pub num_trades: i64,
  pub taker_buy_base_volume: f64,
  pub taker_buy_quote_volume: f64
}

#[derive(Debug, Default)]
struct MaximumAttemptExceeded;

impl Display for MaximumAttemptExceeded {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Maximum retrieving count exceeded.");
  }
}

impl Error for MaximumAttemptExceeded {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

#[derive(Debug, Clone)]
pub struct Binance {
  logger: Logger,
}

impl Binance {
  fn new(logger: Logger) -> Self {
    return Self{logger};
  }
  fn get_ws_endpoint(&self) -> ParseURLResult {
    return "wss://stream.binance.com:9443".parse();
  }
  fn get_rest_endpoint(&self) -> ParseURLResult {
    return "https://api.binance.com".parse();
  }
  async fn get_hist(
    self,
    pair: String,
    startAt: DateTime<Utc>,
    endAt: DateTime<Utc>,
  ) -> GenericResult<BinaceKlineResults> {
    let limit = (endAt - startAt).num_minutes();
    let url = self.get_rest_endpoint()?;
    let mut url = url.join("/api/v3/klines")?;
    let param = HistQuery{
      symbol: pair,
      interval: "1m".into(),
      start_time: format!("{}",startAt.timestamp()),
      end_time: format!("{}", endAt.timestamp()),
      limit: format!("{}", limit),
    };
    let param = to_string(&param)?;
    url.set_query(Some(&param));
    let mut c: i8 = 0;
    while c < 20 {
      let resp = ::reqwest::get(url.clone()).await?;
      let rest_status = resp.status();
      if rest_status.is_success() {
        let values = resp.json::<BinancePayload>().await?;
        let ret: BinaceKlineResults = values.iter().map(|item| {
            return Ok(Kline{
              open_time: cast_datetime("open_time", item[0].clone())?,
              open_price: cast_f64("open_price", item[1].clone())?,
              high_price: cast_f64("high_price", item[2].clone())?,
              low_price: cast_f64("low_price", item[3].clone())?,
              close_price: cast_f64("close_price", item[4].clone())?,
              volume: cast_f64("volume",item[5].clone())?,
              close_time: cast_datetime("close_time", item[6].clone())?,
              quote_volume: cast_f64("quote_volume", item[7].clone())?,
              num_trades: cast_i64("num_trades", item[8].clone())?,
              taker_buy_base_volume:  cast_f64(
                "taker_buy_base_volume", item[9].clone()
              )?,
              taker_buy_quote_volume:  cast_f64(
                "taker_buy_quote_volume", item[10].clone()
              )?,
            });
          }).collect();
        return Ok(ret);
      } else if rest_status == ::reqwest::StatusCode::IM_A_TEAPOT ||
          rest_status == ::reqwest::StatusCode::TOO_MANY_REQUESTS {
        let retry_secs: i64 = match resp.headers().get("retry-after") {
          Some(t) => i64::from_str_radix(
            t.to_str().unwrap_or(&DEFAULT_RECONNECT_INTERVAL.to_string()), 10
          ).unwrap_or(DEFAULT_RECONNECT_INTERVAL),
          None => DEFAULT_RECONNECT_INTERVAL,
        };
        let retry_secs = Duration::seconds(retry_secs);
        ::async_std::task::sleep(retry_secs.to_std()?).await;
        warn!(
          self.logger, "Got code {}. Waiting for {} seconds...",
          rest_status.as_u16(), retry_secs.num_seconds(),
        );
      } else {
        let text = resp.text().await?;
        warn!(
          self.logger, "Got code {}.",
          rest_status.as_u16(); "body" => text,
        );
      }
      c += 1;
    }
    return Err(Box::new(MaximumAttemptExceeded::default()));
  }

  pub async fn spawn_history_fetcher(self) -> (
    mpsc::Sender<HistFetcherParam>,
    mpsc::Receiver<GenericResult<BinaceKlineResults>>,
  ) {
    let (param_send, mut param_rec) = mpsc::channel::<HistFetcherParam>(CHAN_BUF_SIZE);
    let (mut prog_send, prog_rec) = mpsc::channel(CHAN_BUF_SIZE);
    let mut param_option  = param_rec.recv().await;
    async move {
      loop {
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
              match self.clone().get_hist(
                param.symbol.clone(), start_time, end_time
              ).await {
                Err(err) => {
                  prog_send.send(Err(err));
                },
                Ok(ret) => {
                  prog_send.send(Ok(ret));
                },
              }
            }
          },
          None => break,
        }
        param_option = param_rec.recv().await;
      }
    };
    return (param_send, prog_rec);
  }
}

#[async_trait]
impl Exchange for Binance {
  async fn refresh_historical(&self, symbol: String) -> mpsc::Receiver<HistChartProg> {
  }
  async fn refresh_symbols(&self) -> mpsc::Receiver<Vec<SymbolInfo>> {
  }
}
