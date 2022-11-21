use ::std::fmt::Debug;
use ::std::time::Duration as StdDur;
use std::time::SystemTime;

use ::async_trait::async_trait;
use ::log::warn;
use ::rand::random;
use ::serde_qs::to_string as to_qs;
use ::tokio::time::sleep;
use ::url::Url;
use entities::TradeTimeTrait;

use ::clients::binance::REST_ENDPOINT;
use ::config::DEFAULT_RECONNECT_INTERVAL;
use ::entities::HistoryFetchRequest;
use ::errors::{ExecutionFailed, MaximumAttemptExceeded};
use ::rpc::entities::Exchanges;
use ::types::{GenericResult, ThreadSafeResult};

use super::entities::{BinancePayload, Kline, Query};
use crate::entities::KlinesByExchange;
use crate::traits::HistoryFetcher as HistoryFetcherTrait;

#[derive(Debug, Clone)]
pub struct HistoryFetcher {
  pub num_reconnect: i8,
  endpoint: Url,
}

impl HistoryFetcher {
  pub fn new(num_reconnect: Option<i8>) -> GenericResult<Self> {
    return Ok(Self {
      num_reconnect: num_reconnect.unwrap_or(20),
      endpoint: (String::from(REST_ENDPOINT) + "/api/v3/klines").parse()?,
    });
  }

  fn validate_request(
    &self,
    req: &HistoryFetchRequest,
  ) -> ThreadSafeResult<()> {
    if let Some(duration) = req.duration() {
      if duration > StdDur::from_secs(60000) {
        return Err(Box::new(ExecutionFailed::new(
          "The duration must be less than or qeual to 1000 munites.",
        )));
      }
    }
    return Ok(());
  }
}

#[async_trait]
impl HistoryFetcherTrait for HistoryFetcher {
  // type Kline = Kline;
  async fn fetch(
    &self,
    req: &HistoryFetchRequest,
  ) -> ThreadSafeResult<KlinesByExchange> {
    if let Err(e) = self.validate_request(req) {
      return Err(e);
    }
    let retry_status_list = [
      ::reqwest::StatusCode::IM_A_TEAPOT,
      ::reqwest::StatusCode::TOO_MANY_REQUESTS,
    ];
    let mut url = self.endpoint.clone();
    let query: Query = req.into();
    let query = to_qs(&query)?;
    url.set_query(Some(&query));
    for _ in 0..self.num_reconnect {
      let resp = ::reqwest::get(url.clone()).await?;
      let status = resp.status();
      if status.is_success() {
        let payload = resp.json::<BinancePayload>().await?;
        let klines: KlinesByExchange = KlinesByExchange::Binance(
          payload
            .iter()
            .filter_map(|item| match Kline::new(req.symbol.clone(), item) {
              Err(err) => {
                warn!(
                  error = format!("{}", err),
                  symbol = req.symbol;
                  "Failed to fetch Kline data",
                );
                return None;
              }
              Ok(v) => Some(v),
            })
            .collect(),
        );
        return Ok(klines);
      } else if retry_status_list.contains(&status) {
        let mut retry_secs: i64 = resp
          .headers()
          .get("retry-after")
          .map(|v| v.to_str().unwrap_or("0"))
          .map(|v| i64::from_str_radix(v, 10))
          .unwrap_or(Ok(DEFAULT_RECONNECT_INTERVAL))
          .unwrap_or(DEFAULT_RECONNECT_INTERVAL);
        if retry_secs <= 0 {
          retry_secs = DEFAULT_RECONNECT_INTERVAL;
        }
        let retry_secs = StdDur::from_secs(retry_secs.abs() as u64);
        warn!(
          "Got code {}. Waiting for {} seconds...",
          status.as_u16(),
          retry_secs.as_secs(),
        );
        sleep(retry_secs).await;
      } else {
        let text = resp.text().await?;
        warn!(
          body = text,
          code = status.as_u16();
          "Unexpected Payload.",
        );
      }
      let wait_dur = StdDur::from_nanos((random::<u64>() + 1) % 1_000_000);
      sleep(wait_dur).await;
    }
    return Err(Box::new(MaximumAttemptExceeded::default()));
  }

  async fn first_trade_date(
    &self,
    symbol: &str,
  ) -> ThreadSafeResult<SystemTime> {
    let req = HistoryFetchRequest::new(
      Exchanges::Binance,
      symbol,
      Some(SystemTime::UNIX_EPOCH.into()),
      None,
    );
    let KlinesByExchange::Binance(klines) = self.fetch(&req).await?;
    let result = klines
      .into_iter()
      .min_by_key(|kline| kline.open_time())
      .map(|kline| kline.open_time())
      .unwrap_or(SystemTime::now());
    return Ok(result);
  }
}
