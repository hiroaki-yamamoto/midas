use ::std::collections::HashMap;
use ::std::fmt::Debug;
use ::std::iter::FromIterator;

use ::async_trait::async_trait;
use ::chrono::{DateTime, Duration, NaiveDateTime, Utc};
use ::futures::future::{join_all, FutureExt};
use ::mongodb::bson::DateTime as MongoDateTime;
use ::nats::asynk::{Connection, Subscription};
use ::rmp_serde::{from_slice as from_msgpack, to_vec as to_msgpack};
use ::serde_qs::to_string;
use ::tokio::select;
use ::tokio::stream::StreamExt as TokioStreamExt;
use ::tokio::sync::broadcast;
use ::tokio::time::delay_for;
use ::url::Url;

use ::config::{
  CHAN_BUF_SIZE, DEFAULT_RECONNECT_INTERVAL, NUM_OBJECTS_TO_FETCH,
};
use ::mongodb::bson::{doc, Document};
use ::rpc::historical::HistChartProg;
use ::slog::{crit, warn, Logger};
use ::types::{ret_on_err, GenericResult, SendableErrorResult};

use crate::entities::KlineCtrl;
use crate::errors::{EmptyError, MaximumAttemptExceeded};
use crate::traits::HistoryFetcher as HistoryFetcherTrait;

use super::constants::{
  HIST_FETCHER_FETCH_PROG_SUB_NAME, HIST_FETCHER_FETCH_RESP_SUB_NAME,
  HIST_FETCHER_PARAM_SUB_NAME, HIST_RECORDER_LATEST_TRADE_DATE_SUB_NAME,
  REST_ENDPOINT,
};
use super::entities::{
  BinancePayload, HistFetcherParam, HistQuery, Kline, Klines, KlinesWithInfo,
  LatestTradeTime,
};
use super::symbol_fetcher::SymbolFetcher;

#[derive(Debug, Clone)]
pub struct HistoryFetcher {
  pub num_reconnect: i8,
  logger: Logger,
  endpoint: Url,
  broker: Connection,
  symbol_fetcher: SymbolFetcher,
  stop_signal: broadcast::Sender<()>,
}

impl HistoryFetcher {
  pub async fn new(
    num_reconnect: Option<i8>,
    logger: Logger,
    broker: Connection,
    symbol_fetcher: SymbolFetcher,
  ) -> GenericResult<Self> {
    let stop_signal =
      Self::subscribe_kline_ctrl(logger.clone(), broker.clone()).await;
    return Ok(Self {
      num_reconnect: num_reconnect.unwrap_or(20),
      endpoint: (String::from(REST_ENDPOINT) + "/api/v3/klines").parse()?,
      logger,
      broker,
      symbol_fetcher,
      stop_signal,
    });
  }

  async fn subscribe_kline_ctrl(
    logger: Logger,
    broker: Connection,
  ) -> broadcast::Sender<()> {
    let (stop_sender, _) = broadcast::channel(CHAN_BUF_SIZE);
    let stop_sender_thread = stop_sender.clone();
    ::tokio::spawn(async move {
      let stream = match broker.subscribe("binance.kline.ctrl").await {
        Err(e) => {
          crit!(
            logger,
            "Failed to subscribe Binance Kline Control Signal channel: {}",
            e
          );
          return;
        }
        Ok(v) => v,
      };
      let mut stream =
        stream.map(|msg| from_msgpack::<KlineCtrl>(&msg.data[..]));
      loop {
        if let Some(Ok(signal)) = stream.next().await {
          match signal {
            KlineCtrl::Stop => {
              let _ = stop_sender_thread.send(());
            }
          }
        }
      }
    });
    return stop_sender;
  }

  pub async fn fetch(
    &self,
    pair: String,
    start_at: DateTime<Utc>,
    end_at: Option<DateTime<Utc>>,
  ) -> SendableErrorResult<Klines> {
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
        let retry_secs = Duration::seconds(retry_secs);
        warn!(
          self.logger,
          "Got code {}. Waiting for {} seconds...",
          rest_status.as_u16(),
          retry_secs.num_seconds(),
        );
        delay_for(ret_on_err!(retry_secs.to_std())).await;
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

  async fn get_first_trade_date(
    &self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<HashMap<String, LatestTradeTime<DateTime<Utc>>>> {
    let symbols_len = symbols.len() as i64;
    let latest_kline = ret_on_err!(
      self
        .broker
        .request(
          HIST_RECORDER_LATEST_TRADE_DATE_SUB_NAME,
          ret_on_err!(to_msgpack(&symbols)),
        )
        .await
    );
    let mut latest_kline: HashMap<String, LatestTradeTime<MongoDateTime>> =
      ret_on_err!(from_msgpack(&latest_kline.data[..]));
    let first_trade_date_prog = HistChartProg {
      symbol: String::from("Currency Trade Date Fetch"),
      num_symbols: symbols_len,
      cur_symbol_num: 0,
      num_objects: symbols_len,
      cur_object_num: latest_kline.len() as i64,
    };
    let msg = ret_on_err!(to_msgpack(&first_trade_date_prog));
    let _ = self
      .broker
      .publish(HIST_FETCHER_FETCH_PROG_SUB_NAME, msg.as_slice())
      .await;
    let latest_kline_clone = latest_kline.clone();
    let to_fetch_binance = symbols
      .into_iter()
      .filter(move |symbol| !latest_kline_clone.contains_key(symbol));
    let broker = self.broker.clone();
    let logger = self.logger.clone();
    let mut resp_vec = vec![];
    for symbol in to_fetch_binance {
      let param = HistFetcherParam {
        symbol: symbol.clone(),
        num_symbols: 1,
        entire_data_len: 1,
        start_time: DateTime::from_utc(
          NaiveDateTime::from_timestamp(0, 0),
          Utc,
        )
        .into(),
        end_time: None,
      };
      let req_payload = match to_msgpack(&param) {
        Err(e) => {
          crit!(
            logger,
            "Failed to serialize the request to fetch
                the first trade date: {}",
            e
          );
          continue;
        }
        Ok(v) => v,
      };
      resp_vec.push(broker.request(
        HIST_FETCHER_PARAM_SUB_NAME,
        req_payload.as_slice().to_owned(),
      ));
    }
    let first_klines = join_all(resp_vec)
      .map(|item| {
        item
          .into_iter()
          .filter_map(|v| v.ok())
          .map(|v| from_msgpack::<Klines>(&v.data[..]))
          .filter_map(|v| v.ok())
          .filter_map(|mut v| v.pop())
      })
      .await;
    for first_kline in first_klines {
      latest_kline.insert(first_kline.symbol.clone(), first_kline.into());
    }
    return Ok(HashMap::from_iter(latest_kline.iter().map(
      move |(sym, trade_time)| {
        let trade_time: LatestTradeTime<DateTime<Utc>> = trade_time.into();
        return (sym.clone(), trade_time);
      },
    )));
  }
}

#[async_trait]
impl HistoryFetcherTrait for HistoryFetcher {
  async fn refresh(
    &self,
    symbol: Vec<String>,
  ) -> SendableErrorResult<Subscription> {
    if symbol.len() < 1 {
      return Err(Box::new(EmptyError {
        field: String::from("symbol"),
      }));
    }
    let prog_sub = ret_on_err!(
      self
        .broker
        .subscribe(HIST_FETCHER_FETCH_PROG_SUB_NAME)
        .await
    );
    let mut query: Option<Document> = None;
    if symbol[0] != "all" {
      query = Some(doc! { "symbol": doc! { "$in": symbol } });
    }
    let symbols = self.symbol_fetcher.get(query).await?;
    let symbols_len = symbols.len();
    let me = self.clone();
    ::tokio::spawn(async move {
      let end_at = Utc::now();
      let mut stop_ch = me.stop_signal.subscribe();
      let trade_dates = match me
        .get_first_trade_date(
          symbols.into_iter().map(|sym| sym.symbol).collect(),
        )
        .await
      {
        Err(e) => {
          crit!(me.logger, "Failed to fetch the first trade date: {}", e);
          return;
        }
        Ok(v) => v,
      };
      for (symbol, dates) in trade_dates.into_iter() {
        let start_at = dates.open_time;
        let mut entire_data_len = (end_at - start_at).num_minutes();
        let entire_data_len_rem = entire_data_len % NUM_OBJECTS_TO_FETCH as i64;
        entire_data_len /= 1000;
        if entire_data_len_rem > 0 {
          entire_data_len += 1;
        }
        let sec_start_date = start_at;
        let stop_ch = &mut stop_ch;
        while sec_start_date < end_at {
          if let Ok(_) = stop_ch.try_recv() {
            return;
          }
          let mut sec_end_date =
            sec_start_date + Duration::minutes(NUM_OBJECTS_TO_FETCH as i64);
          if sec_end_date > end_at {
            sec_end_date = end_at;
          }
          let msg = match to_msgpack(
            HistFetcherParam {
              symbol: symbol.clone(),
              num_symbols: symbols_len as i64,
              entire_data_len,
              start_time: sec_start_date.clone().into(),
              end_time: Some(sec_end_date.into()),
            }
            .as_ref(),
          ) {
            Err(e) => {
              warn!(me.logger, "Filed to encode HistFetcherParam: {}", e);
              continue;
            }
            Ok(v) => v,
          };
          let _ = me
            .broker
            .publish(HIST_FETCHER_FETCH_RESP_SUB_NAME, msg)
            .await;
        }
      }
    });
    return Ok(prog_sub);
  }

  async fn spawn(&self) -> SendableErrorResult<()> {
    let me = self.clone();
    let mut param_sub = ret_on_err!(
      me.broker
        .queue_subscribe(HIST_FETCHER_PARAM_SUB_NAME, "fetch.thread")
        .await
    )
    .map(|item| (from_msgpack::<HistFetcherParam>(item.data.as_ref()), item));
    ::tokio::spawn(async move {
      loop {
        select! {
          Some((param, msg)) = param_sub.next() => {
            let param = match param {
              Err(e) => {
                warn!(me.logger, "Failed to parse the param msg: {}", e);
                continue;
              },
              Ok(v) => v
            };
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
            );
            let resp = resp.await;
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
                return;
              },
              Ok(v) => v
            };
            match msg.reply {
              Some(_) => {
                let _ = msg.respond(response_payload.as_slice().to_owned()).await;
              },
              None => {
                let _ = me.broker.publish(
                  HIST_FETCHER_FETCH_RESP_SUB_NAME,
                  response_payload.as_slice().to_owned()).await;
              },
            };
          },
          else => {break;}
        }
      }
    });
    return Ok(());
  }

  async fn stop(&self) -> SendableErrorResult<()> {
    let msg = ret_on_err!(to_msgpack(&KlineCtrl::Stop));
    ret_on_err!(self.broker.publish("binance.kline.ctrl", &msg[..]).await);
    return Ok(());
  }
}
