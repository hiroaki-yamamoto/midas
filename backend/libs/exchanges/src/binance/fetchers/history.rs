use ::std::collections::HashMap;
use ::std::fmt::Debug;
use ::std::time::Duration as StdDur;

use ::async_trait::async_trait;
use ::chrono::{DateTime, Duration, NaiveDateTime, Utc};
use ::futures::future::{join_all, FutureExt};
use ::futures::StreamExt;
use ::mongodb::bson::DateTime as MongoDateTime;
use ::nats::asynk::{Connection, Subscription};
use ::rand::random;
use ::rmp_serde::{from_slice as from_msgpack, to_vec as to_msgpack};
use ::serde_qs::to_string;
use ::tokio::select;
use ::tokio::sync::broadcast;
use ::tokio::time::sleep;
use ::url::Url;

use ::config::{DEFAULT_RECONNECT_INTERVAL, NUM_OBJECTS_TO_FETCH};
use ::mongodb::bson::{doc, Document};
use ::rpc::entities::SymbolInfo;
use ::rpc::historical::HistChartProg;
use ::slog::{warn, Logger};
use ::types::{GenericResult, ThreadSafeResult};

use crate::entities::KlineCtrl;
use crate::errors::{EmptyError, MaximumAttemptExceeded, ObjectNotFound};
use crate::traits::HistoryFetcher as HistoryFetcherTrait;

use super::super::constants::{
  HIST_FETCHER_FETCH_PROG_SUB_NAME, HIST_FETCHER_FETCH_REQ_SUB_NAME,
  HIST_FETCHER_FETCH_RESP_SUB_NAME, HIST_FETCHER_PARAM_SUB_NAME,
  HIST_RECORDER_LATEST_TRADE_DATE_SUB_NAME, REST_ENDPOINT,
};
use super::super::entities::{
  BinancePayload, HistFetcherParam, HistQuery, Kline, Klines, KlinesWithInfo,
  TradeTime,
};
use super::SymbolFetcher;

#[derive(Debug, Clone)]
pub struct HistoryFetcher {
  pub num_reconnect: i8,
  logger: Logger,
  endpoint: Url,
  broker: Connection,
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
      broker,
      symbol_fetcher,
    });
  }

  pub async fn fetch(
    &self,
    pair: String,
    start_at: DateTime<Utc>,
    end_at: Option<DateTime<Utc>>,
  ) -> ThreadSafeResult<Klines> {
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
    let param = to_string(&param)?;
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
    symbol: &String,
    symbols_len: usize,
  ) -> GenericResult<TradeTime<DateTime<Utc>>> {
    let latest_klines = self
      .broker
      .request(
        HIST_RECORDER_LATEST_TRADE_DATE_SUB_NAME,
        to_msgpack(&[symbol])?,
      )
      .await?;
    let mut latest_kline: HashMap<String, TradeTime<MongoDateTime>> =
      from_msgpack(&latest_klines.data[..])?;
    let first_trade_date_prog = HistChartProg {
      symbol: String::from("Currency Trade Date Fetch"),
      num_symbols: symbols_len as u64,
      cur_symbol_num: 0,
      num_objects: symbols_len as u64,
      cur_object_num: latest_kline.len() as u64,
    };
    let msg = to_msgpack(&first_trade_date_prog)?;
    let _ = self
      .broker
      .publish(HIST_FETCHER_FETCH_PROG_SUB_NAME, msg.as_slice())
      .await;
    let logger = self.logger.clone();
    let param = HistFetcherParam {
      symbol: symbol.clone(),
      num_symbols: 1,
      entire_data_len: 1,
      start_time: DateTime::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)
        .into(),
      end_time: None,
    };
    let req_payload = to_msgpack(&param)?;
    let logger = logger.clone();
    let resp = self
      .broker
      .request(
        HIST_FETCHER_PARAM_SUB_NAME,
        req_payload.as_slice().to_owned(),
      )
      .then({
        let broker = self.broker.clone();
        |item| async move {
          let item = item
            .map_err(|e| {
              warn!(logger, "Failed to publish the messgae: {}", e);
            })
            .map(|item| {
              from_msgpack::<KlinesWithInfo>(&item.data[..])
                .map_err(|e| {
                  warn!(logger, "Failed to decode the response: {}", e);
                })
                .ok()
            })
            .ok()
            .flatten()
            .map(|mut item| item.klines.pop())
            .map(|item| {
              return item.zip(Some(HistChartProg {
                symbol: String::from("Currency Trade Date Fetch"),
                num_symbols: symbols_len as u64,
                cur_symbol_num: 0,
                num_objects: symbols_len as u64,
                cur_object_num: 1,
              }));
            })
            .flatten()
            .map(|(item, prog)| {
              let msg = to_msgpack(&prog);
              return Some((item, prog)).zip(msg.ok());
            })
            .flatten();
          match item {
            None => {
              warn!(logger, "No value in the response.");
              return None;
            }
            Some(((item, _), msg)) => {
              let _ = broker
                .publish(HIST_FETCHER_FETCH_PROG_SUB_NAME, &msg[..])
                .await;
              return Some(item);
            }
          }
        }
      })
      .boxed();
    let result = resp.await;
    if let Some(result) = result {
      latest_kline.insert(result.symbol.clone(), result.into());
    }
    if let Some(latest_kline) = latest_kline.get(symbol) {
      let latest_kline: TradeTime<DateTime<Utc>> = latest_kline.into();
      return Ok(latest_kline);
    }
    return Err(ObjectNotFound::new("LatestKline".to_string()).into());
  }

  async fn push_fetch_request(
    &self,
    symbol: SymbolInfo,
    symbols_len: usize,
  ) -> GenericResult<()> {
    let end_at = Utc::now();
    let trade_date = self
      .get_first_trade_date(&symbol.symbol, symbols_len)
      .await?;
    let start_at = trade_date.open_time;
    let mut entire_data_len = (end_at - start_at).num_minutes();
    let entire_data_len_rem = entire_data_len % NUM_OBJECTS_TO_FETCH as i64;
    entire_data_len /= 1000;
    if entire_data_len_rem > 0 {
      entire_data_len += 1;
    }
    let mut sec_start_date = start_at;
    let broker = self.broker.clone();
    let logger = self.logger.clone();
    ::tokio::spawn(async move {
      while sec_start_date < end_at {
        let mut sec_end_date =
          sec_start_date + Duration::minutes(NUM_OBJECTS_TO_FETCH as i64);
        if sec_end_date > end_at {
          sec_end_date = end_at;
        }
        let msg = match to_msgpack(
          HistFetcherParam {
            symbol: symbol.symbol.clone(),
            num_symbols: symbols_len as i64,
            entire_data_len,
            start_time: sec_start_date.clone().into(),
            end_time: Some(sec_end_date.into()),
          }
          .as_ref(),
        ) {
          Err(e) => {
            warn!(logger, "Filed to encode HistFetcherParam: {}", e);
            continue;
          }
          Ok(v) => v,
        };
        let _ = broker.publish(HIST_FETCHER_PARAM_SUB_NAME, msg).await;
        sec_start_date = sec_end_date.clone();
      }
    });
    return Ok(());
  }
}

#[async_trait]
impl HistoryFetcherTrait for HistoryFetcher {
  async fn refresh(
    &self,
    symbol: Vec<String>,
  ) -> ThreadSafeResult<Subscription> {
    if symbol.len() < 1 {
      return Err(Box::new(EmptyError {
        field: String::from("symbol"),
      }));
    }
    let mut query: Option<Document> = None;
    if symbol[0] != "all" {
      query = Some(doc! { "symbol": doc! { "$in": symbol } });
    }
    let symbols = self.symbol_fetcher.get(query).await?;
    let symbols_len = symbols.len();
    let symbols = symbols
      .into_iter()
      .map(move |symbol| (symbol, symbols_len.clone()))
      .map(|symbol_tup| to_msgpack(&symbol_tup))
      .filter_map(|res| res.ok());
    let me = self.clone();
    ::tokio::spawn(async move {
      for symbol in symbols {
        let _ = me
          .broker
          .publish(HIST_FETCHER_FETCH_REQ_SUB_NAME, symbol)
          .await;
      }
    });
    return Ok(
      self
        .broker
        .subscribe(HIST_FETCHER_FETCH_PROG_SUB_NAME)
        .await?,
    );
  }

  async fn spawn(&self) -> ThreadSafeResult<()> {
    let me = self.clone();
    let (param_sub, req_sub, ctrl_sub) = (
      self
        .broker
        .queue_subscribe(HIST_FETCHER_PARAM_SUB_NAME, "fetch.thread"),
      self
        .broker
        .queue_subscribe(HIST_FETCHER_FETCH_REQ_SUB_NAME, "fetch.thread"),
      self.broker.subscribe("binance.kline.ctrl"),
    );
    let mut param_sub = param_sub
      .await?
      .map(|item| {
        (from_msgpack::<HistFetcherParam>(item.data.as_ref())
          .ok()
          .zip(Some(item)))
      })
      .filter_map(|msg| async { msg })
      .boxed();
    let mut req_sub = req_sub
      .await?
      .map(|msg| from_msgpack::<(SymbolInfo, usize)>(msg.data.as_ref()))
      .filter_map(|msg| async { msg.ok() })
      .boxed();
    let mut ctrl_sub = ctrl_sub
      .await?
      .map(|msg| from_msgpack::<KlineCtrl>(msg.data.as_ref()))
      .filter_map(|msg| async { msg.ok() })
      .boxed();
    let (stop_send, _) = broadcast::channel::<()>(1024);
    let logger = self.logger.clone();
    loop {
      select! {
        Some(ctrl) = ctrl_sub.next() => {
          match ctrl {
            KlineCtrl::Stop => {
              warn!(
                logger,
                "Stop signal has been received. Stopping the worker..."
              );
              let _ = stop_send.send(());
              break;
            }
          }
        }
        Some((symbol, symbols_len)) = req_sub.next() => {
          let _ = me.push_fetch_request(symbol, symbols_len).await;
        }
        Some((param, msg)) = param_sub.next() => {
          let num_obj = match param.end_time {
            None => 1,
            Some(end_time) => (*end_time - *param.start_time).num_minutes()
          };
          if num_obj > NUM_OBJECTS_TO_FETCH as i64 {
            warn!(
              me.logger,
              "Duration between the specified start and end time exceeds
                the maximum number of the objects to fetch.";
              "symbol" => &param.symbol,
              "start_time" => param.start_time.to_string(),
              "end_time" => format!("{:?}", param.end_time),
              "num_objects" => num_obj,
              "maximum_number_objects" => NUM_OBJECTS_TO_FETCH,
            );
            continue;
          }
          let start_time = *param.start_time;
          let resp = me.fetch(
            param.symbol.clone(),
            start_time,
            param.end_time.map(|d| *d),
          ).await;
          let resp = match resp {
            Err(e) => {
              warn!(me.logger, "Failed to fetch kline data: {}", e);
              continue;
            },
            Ok(v) => v
          };
          let response_payload = match to_msgpack(KlinesWithInfo{
            klines: resp,
            symbol: param.symbol.clone(),
            num_symbols: param.num_symbols,
            entire_data_len: param.entire_data_len
          }.as_ref()) {
            Err(e) => {
              warn!(
                me.logger,
                "Failed to serialize the payload for response: {}", e
              );
              continue;
            },
            Ok(v) => v
          };
          match msg.reply {
            Some(_) => {
              if let Err(e) = msg.respond(&response_payload[..].to_owned()).await {
                warn!(logger, "Failed to respond to the request: {}", e;
                "subject" => msg.subject, "reply" => msg.reply);
              }
            },
            None => {
              let _ = me.broker.publish(
                HIST_FETCHER_FETCH_RESP_SUB_NAME,
                &response_payload[..].to_owned()).await;
            },
          };
        },
        else => {break;}
      }
    }
    return Ok(());
  }

  async fn stop(&self) -> ThreadSafeResult<()> {
    let msg = to_msgpack(&KlineCtrl::Stop)?;
    self.broker.publish("binance.kline.ctrl", &msg[..]).await?;
    return Ok(());
  }
}
