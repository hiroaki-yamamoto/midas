use ::chrono::{DateTime, NaiveDateTime, Utc};
use ::serde::Serialize;
use ::serde_json::Value;
use ::serde_qs::to_string;
use ::std::error::Error;
use ::types::ParseURLResult;

use crate::traits::Exchange;

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
  ) -> Result<Vec<Kline>, Box<dyn Error>> {
    let mut limit = (endAt - startAt).num_minutes();
    if limit <= 1000 {
      limit = 1000;
    }
    let url = self.get_rest_endpoint()?;
    let url = url.join("/api/v3/klines")?;
    let param = HistQuery{
      symbol: pair,
      interval: "1m".into(),
      start_time: format!("{}",startAt.timestamp()),
      end_time: format!("{}", endAt.timestamp()),
      limit: format!("{}", limit),
    };
    let param = to_string(&param)?;
    url.set_query(Some(&param));
    let c: i8 = 0;
    while c < 20 {
      let resp = ::reqwest::get(url).await?;
      if resp.status().is_success() {
        let values = resp.json::<BinancePayload>().await?;
        let ret = values.iter().map(|item|{
          let open_time = match item[0].as_i64() {
            Some(n) => n,
            None => return Err("Failed to parse open_time"),
          };
          let open_price: f64 = match item[1].as_str() {
            Some(s) => s.parse()?,
            None => return Err("Failed to parse open_price"),
          };
          Kline{
            open_time: DateTime::from_utc(
              NaiveDateTime::from_timestamp(open_time, 0), Utc,
            ),
            open_price,
          }
        });
      }
      c += 1;
    }
    return Ok(vec![Kline{}]);
  }
}

impl Exchange for Binance {
  async fn refresh_historical(&self, symbol: String) -> Receiver<HistChartProg> {
  }
  async fn refresh_symbols(&self) -> Receiver<Vec<SymbolInfo>> {
  }
}
