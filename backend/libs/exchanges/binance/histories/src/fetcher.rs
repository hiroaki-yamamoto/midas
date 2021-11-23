use ::std::fmt::Debug;
use ::std::time::{Duration as StdDur, SystemTime};

use ::async_trait::async_trait;
use ::mongodb::bson::DateTime as MongoDateTime;
use ::nats::Connection;
use ::rand::random;
use ::serde_qs::to_string as to_qs;
use ::slog::{warn, Logger};
use ::subscribe::PubSub;
use ::tokio::time::sleep;
use ::url::Url;

use ::binance_symbols::fetcher::SymbolFetcher;
use ::config::DEFAULT_RECONNECT_INTERVAL;
use ::entities::HistoryFetchRequest;
use ::errors::{EmptyError, ExecutionFailed, MaximumAttemptExceeded};
use ::history::{HistoryFetcher as HistoryFetcherTrait, KlineTrait};
use ::types::{GenericResult, ThreadSafeResult};

use super::entities::{
  BinancePayload, Kline, KlinesWithInfo, Param, Query, TradeTime,
};
use super::pubsub::{
  HistFetchParamPubSub, HistFetchRespPubSub, HistProgPartPubSub,
  KlineControlPubSub, RecLatestTradeDatePubSub,
};
use ::binance_clients::constants::REST_ENDPOINT;

#[derive(Debug, Clone)]
pub struct HistoryFetcher {
  pub num_reconnect: i8,
  logger: Logger,
  endpoint: Url,
  prog_pubsub: HistProgPartPubSub,
  param_pubsub: HistFetchParamPubSub,
  resp_pubsub: HistFetchRespPubSub,
  rec_ltd_pubsub: RecLatestTradeDatePubSub,
  ctrl_pubsub: KlineControlPubSub,
  symbol_fetcher: SymbolFetcher,
}

impl HistoryFetcher {
  pub async fn new(
    num_reconnect: Option<i8>,
    logger: Logger,
    broker: Connection,
    symbol_fetcher: SymbolFetcher,
  ) -> GenericResult<Self> {
    return Ok(Self {
      num_reconnect: num_reconnect.unwrap_or(20),
      endpoint: (String::from(REST_ENDPOINT) + "/api/v3/klines").parse()?,
      logger,
      prog_pubsub: HistProgPartPubSub::new(broker.clone()),
      param_pubsub: HistFetchParamPubSub::new(broker.clone()),
      resp_pubsub: HistFetchRespPubSub::new(broker.clone()),
      rec_ltd_pubsub: RecLatestTradeDatePubSub::new(broker.clone()),
      ctrl_pubsub: KlineControlPubSub::new(broker.clone()),
      symbol_fetcher,
    });
  }

  async fn get_first_trade_date(
    &self,
    symbol: String,
  ) -> GenericResult<TradeTime<SystemTime>> {
    let db_trade_date_opt = self
      .rec_ltd_pubsub
      .request::<TradeTime<MongoDateTime>>(&symbol)
      .ok();
    let (resp, _): (KlinesWithInfo, _) = self.param_pubsub.request(&Param {
      symbol: symbol.clone(),
      num_symbols: 1,
      entire_data_len: 1,
      start_time: SystemTime::UNIX_EPOCH.into(),
      end_time: None,
    })?;
    let resp: Kline = resp
      .klines
      .first()
      .ok_or(EmptyError {
        field: "klines".to_string(),
      })?
      .clone();
    if let Some((db_recent_trade_date, _)) = db_trade_date_opt {
      if db_recent_trade_date.open_time > resp.open_time {
        return Ok(db_recent_trade_date.into());
      }
    }
    return Ok(resp.into());
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
  async fn fetch<T>(
    &self,
    req: &HistoryFetchRequest,
  ) -> ThreadSafeResult<Vec<T>>
  where
    T: KlineTrait + Clone,
  {
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
    for i in 0..20 {
      let resp = ::reqwest::get(url.clone()).await?;
      let status = resp.status();
      if status.is_success() {
        let payload = resp.json::<BinancePayload>().await?;
        let klines: Vec<T> = payload
          .iter()
          .filter_map(|item| match Kline::new(pair.clone(), item) {
            Err(err) => {
              warn!(
                self.logger,
                "Failed to fetch Kline data: {}",
                err; "symbol" => &pair
              );
              return None;
            }
            Ok(v) => Some(v),
          })
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
