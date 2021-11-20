use ::std::fmt::Debug;
use ::std::time::{Duration as StdDur, SystemTime, UNIX_EPOCH};

use ::async_trait::async_trait;
use ::futures::stream::{BoxStream, StreamExt};
use ::mongodb::bson::{doc, DateTime as MongoDateTime, Document};
use ::nats::{Connection, Message};
use ::rand::random;
use ::serde_qs::to_string as to_qs;
use ::slog::{warn, Logger};
use ::subscribe::PubSub;
use ::tokio::select;
use ::tokio::time::{sleep, timeout};
use ::url::Url;

use ::binance_symbols::fetcher::SymbolFetcher;
use ::config::{DEFAULT_RECONNECT_INTERVAL, NUM_OBJECTS_TO_FETCH};
use ::entities::{HistoryFetchRequest, KlineCtrl};
use ::errors::{EmptyError, ExecutionFailed, MaximumAttemptExceeded};
use ::history::{HistoryFetcher as HistoryFetcherTrait, KlineTrait};
use ::rpc::symbols::SymbolInfo;
use ::types::{GenericResult, ThreadSafeResult};

use super::entities::{
  BinancePayload, Kline, Klines, KlinesWithInfo, Param, Query, TradeTime,
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

  pub async fn fetch(
    &self,
    pair: String,
    start_at: SystemTime,
    end_at: Option<SystemTime>,
  ) -> ThreadSafeResult<Klines> {
    let limit = match end_at {
      Some(end_at) => Some(end_at.duration_since(start_at)?.as_secs() / 60),
      None => None,
    };
    let mut url = self.endpoint.clone();
    let param = Query {
      symbol: pair.clone(),
      interval: "1m".into(),
      start_time: format!(
        "{}",
        start_at.duration_since(UNIX_EPOCH)?.as_millis()
      ),
      end_time: match end_at {
        Some(end_at) => Some(format!(
          "{}",
          end_at.duration_since(UNIX_EPOCH)?.as_millis()
        )),
        None => None,
      },
      limit: format!("{}", limit.unwrap_or(1)),
    };
    let param = to_qs(&param)?;
    url.set_query(Some(&param));
    let mut c: i8 = 0;
    while c < 20 {
      let resp = ::reqwest::get(url.clone()).await?;
      let rest_status = resp.status();
      if rest_status.is_success() {
        let values = resp.json::<BinancePayload>().await?;
        let ret: Klines = values
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
        return Ok(ret);
      } else if rest_status == ::reqwest::StatusCode::IM_A_TEAPOT
        || rest_status == ::reqwest::StatusCode::TOO_MANY_REQUESTS
      {
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
          rest_status.as_u16(),
          retry_secs.as_secs(),
        );
        sleep(retry_secs).await;
      } else {
        let text = resp.text().await?;
        warn!(
          self.logger, "Got code {}.",
          rest_status.as_u16(); "body" => text,
        );
      }
      c += 1;
      let wait_dur = StdDur::from_nanos((random::<u64>() + 1) % 1_000_000);
      sleep(wait_dur).await;
    }
    return Err(Box::new(MaximumAttemptExceeded::default()));
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

  async fn push_fetch_request(
    &self,
    symbols: &Vec<SymbolInfo>,
    stop_sig: &mut BoxStream<'_, (KlineCtrl, Message)>,
  ) -> GenericResult<()> {
    let end_at = SystemTime::now();
    let symbols_len = symbols.len();
    for symbol in symbols {
      let recent_trade_date =
        match self.get_first_trade_date(symbol.symbol.clone()).await {
          Err(_) => {
            continue;
          }
          Ok(d) => d,
        };
      let start_at = recent_trade_date.open_time;
      let mut entire_data_len = end_at.duration_since(start_at)?.as_secs() / 60;
      let entire_data_len_rem = entire_data_len % NUM_OBJECTS_TO_FETCH as u64;
      entire_data_len /= 1000;
      if entire_data_len_rem > 0 {
        entire_data_len += 1;
      }
      let mut sec_start_date = start_at;
      while sec_start_date < end_at {
        if let Ok(Some((ctrl, _))) =
          timeout(StdDur::from_micros(1), stop_sig.next()).await
        {
          match ctrl {
            KlineCtrl::Stop => {
              break;
            }
          }
        }
        let mut sec_end_date = sec_start_date
          + StdDur::from_secs((NUM_OBJECTS_TO_FETCH as u64) * 60);
        if sec_end_date > end_at {
          sec_end_date = end_at;
        }
        let _ = self.param_pubsub.publish(&Param {
          symbol: symbol.symbol.clone(),
          num_symbols: symbols_len as i64,
          entire_data_len,
          start_time: sec_start_date.clone().into(),
          end_time: Some(sec_end_date.into()),
        });
        sec_start_date = sec_end_date.clone();
      }
    }
    return Ok(());
  }
  async fn fetch_inner(
    &self,
    req: &HistoryFetchRequest,
  ) -> ThreadSafeResult<::reqwest::Response> {
    let std_start = req.start.to_system_time();
    let query = match req.end.map(|d| d.to_system_time()) {
      Some(std_end) => {
        let duration = std_end.duration_since(std_start)?;
        // 60000 = 60 secs (i.e. 1 minutes) x 1000 objects.
        if duration > StdDur::from_secs(60000) {
          return Err(Box::new(ExecutionFailed::new(
            "The duration must be less than or qeual to 1000 munites.",
          )));
        }
        Query {
          symbol: req.symbol,
          start_time: std_start
            .duration_since(UNIX_EPOCH)?
            .as_millis()
            .to_string(),
          end_time: Some(
            std_end.duration_since(UNIX_EPOCH)?.as_millis().to_string(),
          ),
          interval: "1m".into(),
          limit: (duration.as_secs() / 60).to_string(),
        }
      }
      None => Query {
        symbol: req.symbol,
        start_time: std_start
          .duration_since(UNIX_EPOCH)?
          .as_millis()
          .to_string(),
        end_time: None,
        interval: "1m".into(),
        limit: "1".into(),
      },
    };
    let mut url = self.endpoint.clone();
    let query = to_qs(&query)?;
    url.set_query(Some(&query));
    return Ok(::reqwest::get(url.clone()).await?);
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
    return Ok(());
  }
}
