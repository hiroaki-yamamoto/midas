use ::chrono::{DateTime, Utc};
use ::serde::Serialize;
use ::serde_json::Value;
use ::serde_qs::to_string;
use ::std::fmt::{Display, Formatter, Result as FormatResult};
use ::std::error::Error;
use ::types::ParseURLResult;

use crate::traits::Exchange;
use crate::casting::{cast_datetime, cast_f64, cast_i64};

type BinancePayload = Vec<Vec<Value>>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HistQuery {
  pub symbol: String,
  pub interval: String,
  pub start_time: String,
  pub end_time: String,
  pub limit: String,
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

pub struct Binance;

impl Binance {
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
  ) -> Result<Vec<Result<Kline, Box<dyn Error>>>, Box<dyn Error>> {
    let mut limit = (endAt - startAt).num_minutes();
    if limit <= 1000 {
      limit = 1000;
    }
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
      if resp.status().is_success() {
        let values = resp.json::<BinancePayload>().await?;
        let ret: Vec<Result<Kline, Box<dyn Error>>> =
          values.iter().map(|item| {
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
      }
      c += 1;
    }
    return Err(Box::new(MaximumAttemptExceeded::default()));
  }
}

impl Exchange for Binance {
  async fn refresh_historical(&self, symbol: String) -> Receiver<HistChartProg> {
  }
  async fn refresh_symbols(&self) -> Receiver<Vec<SymbolInfo>> {
  }
}
