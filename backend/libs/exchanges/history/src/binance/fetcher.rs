use ::std::fmt::Debug;
use ::std::time::Duration as StdDur;

use ::async_trait::async_trait;
use ::rand::random;
use ::serde_qs::to_string as to_qs;
use ::slog::{warn, Logger};
use ::tokio::time::sleep;
use ::url::Url;

use crate::traits::kline::Kline as KlineTrait;
use crate::traits::HistoryFetcher as HistoryFetcherTrait;
use ::config::DEFAULT_RECONNECT_INTERVAL;
use ::entities::HistoryFetchRequest;
use ::errors::{ExecutionFailed, MaximumAttemptExceeded};
use ::types::{GenericResult, ThreadSafeResult};

use super::entities::{BinancePayload, Kline, Query};
use ::clients::binance::REST_ENDPOINT;

#[derive(Debug, Clone)]
pub struct HistoryFetcher {
  pub num_reconnect: i8,
  logger: Logger,
  endpoint: Url,
}

impl HistoryFetcher {
  pub fn new(num_reconnect: Option<i8>, logger: Logger) -> GenericResult<Self> {
    return Ok(Self {
      num_reconnect: num_reconnect.unwrap_or(20),
      endpoint: (String::from(REST_ENDPOINT) + "/api/v3/klines").parse()?,
      logger,
    });
  }

  // async fn get_first_trade_date(
  //   &self,
  //   symbol: String,
  // ) -> GenericResult<TradeTime<SystemTime>> {
  //   let req = &Param {
  //     symbol: symbol.clone(),
  //     num_symbols: 1,
  //     entire_data_len: 1,
  //     start_time: SystemTime::UNIX_EPOCH.into(),
  //     end_time: None,
  //   };
  // }

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
  ) -> ThreadSafeResult<Vec<Box<dyn KlineTrait + Send + Sync>>> {
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
        let klines: Vec<Box<dyn KlineTrait + Send + Sync>> = payload
          .iter()
          .filter_map(|item| match Kline::new(req.symbol.clone(), item) {
            Err(err) => {
              warn!(
                self.logger,
                "Failed to fetch Kline data: {}",
                err; "symbol" => &req.symbol
              );
              return None;
            }
            Ok(v) => Some(v),
          })
          .map(|item| Box::new(item) as Box<dyn KlineTrait + Send + Sync>)
          .collect();
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
          self.logger,
          "Got code {}. Waiting for {} seconds...",
          status.as_u16(),
          retry_secs.as_secs(),
        );
        sleep(retry_secs).await;
      } else {
        let text = resp.text().await?;
        warn!(
          self.logger, "Got code {}.",
          status.as_u16(); "body" => text,
        );
      }
      let wait_dur = StdDur::from_nanos((random::<u64>() + 1) % 1_000_000);
      sleep(wait_dur).await;
    }
    return Err(Box::new(MaximumAttemptExceeded::default()));
  }
}
