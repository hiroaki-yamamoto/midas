use ::std::collections::HashMap;
use ::std::fmt::Debug;
use ::std::io::Result as IOResult;
use ::std::iter::FromIterator;
use ::std::time::Duration as StdDur;

use ::async_trait::async_trait;
use ::chrono::{DateTime, Duration, NaiveDateTime, Utc};
use ::futures::stream::{BoxStream, StreamExt};
use ::mongodb::bson::oid::ObjectId;
use ::mongodb::bson::{doc, DateTime as MongoDateTime, Document};
use ::nats::{Connection, Message};
use ::rand::random;
use ::serde_qs::to_string;
use ::slog::{warn, Logger};
use ::subscribe::PubSub;
use ::tokio::select;
use ::tokio::time::{sleep, timeout};
use ::url::Url;

use ::binance_clients::reqwest;
use ::binance_symbols::fetcher::SymbolFetcher;
use ::config::{DEFAULT_RECONNECT_INTERVAL, NUM_OBJECTS_TO_FETCH};
use ::entities::KlineCtrl;
use ::errors::{EmptyError, MaximumAttemptExceeded};
use ::history::HistoryFetcher as HistoryFetcherTrait;
use ::rpc::entities::SymbolInfo;
use ::rpc::historical::HistChartProg;
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
    start_at: DateTime<Utc>,
    end_at: Option<DateTime<Utc>>,
  ) -> ThreadSafeResult<Klines> {
    let limit = match end_at {
      Some(end_at) => Some((end_at - start_at).num_minutes()),
      None => None,
    };
    let mut url = self.endpoint.clone();
    let param = Query {
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
      let resp = reqwest::get(url.clone()).await?;
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
      } else if rest_status == reqwest::StatusCode::IM_A_TEAPOT
        || rest_status == reqwest::StatusCode::TOO_MANY_REQUESTS
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
    symbols: Vec<String>,
  ) -> GenericResult<HashMap<String, TradeTime<DateTime<Utc>>>> {
    let symbols_len = symbols.len() as i64;
    let (mut latest_kline, _) = self
      .rec_ltd_pubsub
      .request::<HashMap<String, TradeTime<MongoDateTime>>>(&symbols)?;
    let _ = self.prog_pubsub.publish(&HistChartProg {
      id: ObjectId::new().to_string(),
      symbol: String::from("Currency Trade Date Fetch"),
      num_symbols: symbols_len,
      cur_symbol_num: 0,
      num_objects: symbols_len,
      cur_object_num: 0,
    })?;
    let latest_kline_clone = latest_kline.clone();
    let to_fetch_binance = symbols
      .into_iter()
      .filter(move |symbol| !latest_kline_clone.contains_key(symbol));
    let logger = self.logger.clone();
    for symbol in to_fetch_binance {
      let resp: IOResult<(KlinesWithInfo, _)> =
        self.param_pubsub.request(&Param {
          symbol: symbol.clone(),
          num_symbols: 1,
          entire_data_len: 1,
          start_time: DateTime::from_utc(
            NaiveDateTime::from_timestamp(0, 0),
            Utc,
          )
          .into(),
          end_time: None,
        });
      let logger = logger.clone();
      match resp {
        Err(e) => {
          warn!(logger, "Failed to publish the messgae: {}", e);
          continue;
        }
        Ok((mut resp, _)) => {
          let resp: Kline = match resp.klines.pop() {
            None => {
              warn!(logger, "No value in the response.");
              continue;
            }
            Some(kline) => kline,
          };
          let prog = HistChartProg {
            id: ObjectId::new().to_string(),
            symbol: String::from("Currency Trade Date Fetch"),
            num_symbols: symbols_len,
            cur_symbol_num: 0,
            num_objects: symbols_len,
            cur_object_num: 1,
          };
          latest_kline.insert(resp.symbol.clone(), resp.into());
          let _ = self.prog_pubsub.publish(&prog);
        }
      }
    }
    return Ok(HashMap::from_iter(latest_kline.iter().map(
      move |(sym, trade_time)| {
        let trade_time: TradeTime<DateTime<Utc>> = trade_time.into();
        return (sym.clone(), trade_time);
      },
    )));
  }

  async fn push_fetch_request(
    &self,
    symbols: &Vec<SymbolInfo>,
    stop_sig: &mut BoxStream<'_, (KlineCtrl, Message)>,
  ) -> GenericResult<()> {
    let end_at = Utc::now();
    let trade_dates = self
      .get_first_trade_date(
        symbols.into_iter().map(|sym| sym.symbol.clone()).collect(),
      )
      .await?;
    let symbols_len = symbols.len();
    for (symbol, dates) in trade_dates.into_iter() {
      let start_at = dates.open_time;
      let mut entire_data_len = (end_at - start_at).num_minutes();
      let entire_data_len_rem = entire_data_len % NUM_OBJECTS_TO_FETCH as i64;
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
        let mut sec_end_date =
          sec_start_date + Duration::minutes(NUM_OBJECTS_TO_FETCH as i64);
        if sec_end_date > end_at {
          sec_end_date = end_at;
        }
        let _ = self.param_pubsub.publish(&Param {
          symbol: symbol.clone(),
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
}

#[async_trait]
impl HistoryFetcherTrait for HistoryFetcher {
  async fn refresh(&self, symbol: Vec<String>) -> ThreadSafeResult<()> {
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
    let me = self.clone();
    let _ = tokio::spawn(async move {
      let mut ctrl_sub = match me.ctrl_pubsub.subscribe() {
        Err(_) => return,
        Ok(o) => o,
      };
      let _ = me.push_fetch_request(&symbols, &mut ctrl_sub).await;
    });
    return Ok(());
  }

  async fn spawn(&self) -> ThreadSafeResult<()> {
    let param_sub = self.param_pubsub.queue_subscribe("fetch.thread")?;
    let mut ctrl_sub = self.ctrl_pubsub.subscribe()?;
    let mut param_sub = param_sub.boxed();
    loop {
      select! {
        Some((ctrl, _)) = ctrl_sub.next() => {
          match ctrl {
            KlineCtrl::Stop => {
              warn!(self.logger, "Stop Signal has been received. Shutting down the worker...");
              break;
            }
          }
        },
        Some((param, msg)) = param_sub.next() => {
          let num_obj = match param.end_time {
            None => 1,
            Some(end_time) => (*end_time - *param.start_time).num_minutes()
          };
          if num_obj > NUM_OBJECTS_TO_FETCH as i64 {
            warn!(
              self.logger,
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
          let resp = self.fetch(
            param.symbol.clone(),
            start_time,
            param.end_time.map(|d| *d),
          );
          let resp = resp.await;
          let resp = match resp {
            Err(e) => {
              warn!(self.logger, "Failed to fetch kline data: {}", e);
              continue;
            },
            Ok(v) => v
          };
          let payload = &KlinesWithInfo{
            klines: resp,
            symbol: param.symbol.clone(),
            num_symbols: param.num_symbols,
            entire_data_len: param.entire_data_len
          };
          let _ = match msg.reply {
            Some(_) => self.resp_pubsub.respond(&msg, payload),
            None => self.resp_pubsub.publish(payload)
          };
        },
        else => {break;}
      }
    }
    return Ok(());
  }

  async fn stop(&self) -> ThreadSafeResult<()> {
    return Ok(self.ctrl_pubsub.publish(&KlineCtrl::Stop)?);
  }
}
