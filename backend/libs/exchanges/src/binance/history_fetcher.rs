use ::std::thread;

use ::chrono::{DateTime, Duration, Utc};
use ::crossbeam::channel::{bounded, Receiver, Sender};
use ::serde_qs::to_string;
use ::url::Url;

use ::config::{
  CHAN_BUF_SIZE, DEFAULT_RECONNECT_INTERVAL, NUM_OBJECTS_TO_FETCH,
};
use ::slog::{error, warn, Logger};
use ::types::{ret_on_err, SendableErrorResult};

use super::constants::REST_ENDPOINT;
use super::entities::{
  BinancePayload, HistFetcherParam, HistQuery, Kline, KlineResults,
  KlineResultsWithSymbol,
};
use crate::errors::{MaximumAttemptExceeded, NumObjectError};

#[derive(Debug, Clone)]
pub(super) struct HistoryFetcher {
  pub num_reconnect: i8,
  logger: Logger,
  endpoint: Url,
}

impl HistoryFetcher {
  pub fn new(
    num_reconnect: Option<i8>,
    logger: Logger,
  ) -> SendableErrorResult<Self> {
    return Ok(Self {
      num_reconnect: num_reconnect.unwrap_or(20),
      endpoint: ret_on_err!(
        (String::from(REST_ENDPOINT) + "/api/v3/klines").parse()
      ),
      logger,
    });
  }
  pub async fn fetch(
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
    let mut url = self.endpoint.clone();
    let param = HistQuery {
      symbol: pair.clone(),
      interval: "1m".into(),
      start_time: format!("{}", start_at.timestamp() * 1000),
      end_time: match end_at {
        Some(end_at) => Some(format!("{}", end_at.timestamp() * 1000)),
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

  pub fn spawn(
    &self,
    stop: Receiver<()>,
  ) -> (
    Sender<HistFetcherParam>,
    Receiver<SendableErrorResult<KlineResultsWithSymbol>>,
  ) {
    let (param_send, param_rec) = bounded::<HistFetcherParam>(CHAN_BUF_SIZE);
    let (prog_send, prog_rec) =
      bounded::<SendableErrorResult<KlineResultsWithSymbol>>(CHAN_BUF_SIZE);
    let me = self.clone();
    thread::spawn(move || {
      ::tokio::spawn(async move {
        while let Err(_) = stop.try_recv() {
          let param_option = param_rec.recv();
          match param_option {
            Ok(param) => {
              let num_obj = (param.end_time - param.start_time).num_minutes();
              if num_obj > NUM_OBJECTS_TO_FETCH as i64 {
                let _ = prog_send.send(Err(Box::new(NumObjectError {
                  field: String::from("Duration between start and end date"),
                  num_object: NUM_OBJECTS_TO_FETCH,
                })));
                continue;
              }
              let _ = prog_send.send(
                me.fetch(
                  param.symbol.clone(),
                  param.num_symbols,
                  param.entire_data_len,
                  param.start_time,
                  Some(param.end_time),
                )
                .await,
              );
            }
            Err(err) => {
              error!(
                me.logger,
                "Got an error while reading param ch. Err: {}", err
              );
              continue;
            }
          }
        }
      });
    });
    return (param_send, prog_rec);
  }
}
