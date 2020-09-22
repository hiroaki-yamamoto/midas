use ::std::fmt::Debug;

use ::futures::future::FutureExt;

use ::async_trait::async_trait;
use ::chrono::{DateTime, Duration, NaiveDateTime, Utc};
use ::nats::asynk::Connection;
use ::rmp_serde::{from_slice as from_msgpack, to_vec as to_msgpack};
use ::serde_qs::to_string;
use ::tokio::select;
use ::tokio::stream::StreamExt as TokioStreamExt;
use ::tokio::sync::{broadcast, mpsc};
use ::tokio::time::delay_for;
use ::url::Url;

use ::config::{
  CHAN_BUF_SIZE, DEFAULT_RECONNECT_INTERVAL, NUM_CONC_TASKS,
  NUM_OBJECTS_TO_FETCH,
};
use ::mongodb::bson::{doc, Document};
use ::mongodb::Collection;
use ::rpc::historical::HistChartProg;
use ::slog::{crit, warn, Logger};
use ::types::{ret_on_err, GenericResult, SendableErrorResult};

use crate::entities::KlineCtrl;
use crate::errors::{
  DeterminationFailed, EmptyError, MaximumAttemptExceeded, NumObjectError,
};
use crate::traits::HistoryFetcher as HistoryFetcherTrait;

use super::constants::REST_ENDPOINT;
use super::entities::{
  BinancePayload, HistFetcherParam, HistQuery, Kline, KlineResults,
  KlineResultsWithSymbol,
};
use super::history_recorder::HistoryRecorder;
use super::symbol_fetcher::SymbolFetcher;

#[derive(Debug, Clone)]
pub struct HistoryFetcher {
  pub num_reconnect: i8,
  recorder: HistoryRecorder,
  logger: Logger,
  endpoint: Url,
  broker: Connection,
  symbol_fetcher: SymbolFetcher,
  stop_signal: broadcast::Sender<()>,
}

impl HistoryFetcher {
  pub async fn new(
    num_reconnect: Option<i8>,
    col: Collection,
    logger: Logger,
    broker: Connection,
    symbol_fetcher: SymbolFetcher,
  ) -> GenericResult<Self> {
    let stop_signal =
      Self::subscribe_kline_ctrl(logger.clone(), broker.clone()).await;
    return Ok(Self {
      num_reconnect: num_reconnect.unwrap_or(20),
      endpoint: (String::from(REST_ENDPOINT) + "/api/v3/klines").parse()?,
      recorder: HistoryRecorder::new(col, &stop_signal),
      logger,
      broker,
      symbol_fetcher,
      stop_signal: stop_signal.clone(),
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

  pub fn spawn(
    &self,
    mut stop: broadcast::Receiver<()>,
  ) -> (
    mpsc::UnboundedSender<HistFetcherParam>,
    mpsc::UnboundedReceiver<SendableErrorResult<KlineResultsWithSymbol>>,
  ) {
    let (param_send, mut param_rec) =
      mpsc::unbounded_channel::<HistFetcherParam>();
    let (prog_send, prog_rec) =
      mpsc::unbounded_channel::<SendableErrorResult<KlineResultsWithSymbol>>();
    let me = self.clone();
    ::tokio::spawn(async move {
      loop {
        let prog_send = &prog_send;
        select! {
          _ = stop.recv() => {break;},
          param_option_result = param_rec.recv() => {
            if let Some(param) = param_option_result {
              let num_obj = (param.end_time - param.start_time).num_minutes();
              if num_obj > NUM_OBJECTS_TO_FETCH as i64 {
                let _ = prog_send.send(Err(Box::new(NumObjectError {
                  field: String::from("Duration between start and end date"),
                  num_object: NUM_OBJECTS_TO_FETCH,
                })));
                continue;
              }
              me.fetch(
                param.symbol.clone(),
                param.num_symbols,
                param.entire_data_len,
                param.start_time,
                Some(param.end_time),
              )
              .then(|item| async move {
                let _ = prog_send.send(item);
              })
              .await;
            }
          }
        }
      }
    });
    return (param_send, prog_rec);
  }

  async fn get_first_trade_date(
    &self,
    symbol: &String,
  ) -> SendableErrorResult<DateTime<Utc>> {
    match self.recorder.get_latest_trade_open_time(symbol).await {
      Ok(d) => return Ok(d),
      Err(_) => {
        let start = self
          .fetch(
            symbol.clone(),
            1,
            1,
            DateTime::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            None,
          )
          .await?;
        let mut start = start.klines;
        start.retain(|item| item.is_ok());
        if start.len() != 1 {
          return Err(Box::new(DeterminationFailed::<()> {
            field: String::from("Start Date"),
            additional_data: None,
          }));
        }
        return Ok(start[0].as_ref().unwrap().open_time.into());
      }
    }
  }
}

#[async_trait]
impl HistoryFetcherTrait for HistoryFetcher {
  async fn refresh(
    &self,
    symbol: Vec<String>,
  ) -> SendableErrorResult<
    mpsc::UnboundedReceiver<SendableErrorResult<HistChartProg>>,
  > {
    if symbol.len() < 1 {
      return Err(Box::new(EmptyError {
        field: String::from("symbol"),
      }));
    }
    let (res_send, res_recv) =
      mpsc::unbounded_channel::<SendableErrorResult<HistChartProg>>();
    let mut senders = vec![];
    let mut recvers = vec![];
    for _ in 0..NUM_CONC_TASKS {
      let (param, res) = self.spawn(self.stop_signal.subscribe());
      senders.push(param);
      recvers.push(res);
    }
    for recv_ch in recvers {
      self.recorder.spawn(
        self.stop_signal.subscribe(),
        recv_ch,
        res_send.clone(),
      );
    }
    let mut query: Option<Document> = None;
    if symbol[0] != "all" {
      query = Some(doc! { "symbol": doc! { "$in": symbol } });
    }
    let symbols = self.symbol_fetcher.get(query).await?;
    let symbols_len = symbols.len();
    let me = self.clone();
    let res_send_in_thread = res_send.clone();
    let mut stop = self.stop_signal.subscribe();
    ::tokio::spawn(async move {
      let end_at = Utc::now();
      for symbol in symbols {
        let start_at = match me.get_first_trade_date(&symbol.symbol).await {
          Err(e) => {
            let _ = res_send_in_thread.send(Err(e));
            break;
          }
          Ok(v) => v,
        };
        let mut entire_data_len = (end_at - start_at).num_minutes();
        let entire_data_len_rem = entire_data_len % NUM_OBJECTS_TO_FETCH as i64;
        entire_data_len /= 1000;
        if entire_data_len_rem > 0 {
          entire_data_len += 1;
        }
        let mut sec_end_date = end_at;
        let mut index = 0;
        while sec_end_date > start_at {
          if let Ok(_) = stop.try_recv() {
            return;
          }
          let mut sec_start_date =
            sec_end_date - Duration::minutes(NUM_OBJECTS_TO_FETCH as i64);
          if sec_start_date < start_at {
            sec_start_date = start_at;
          }
          let sender = &mut senders[index];
          let _ = sender.send(HistFetcherParam {
            symbol: symbol.symbol.clone(),
            num_symbols: symbols_len as i64,
            entire_data_len,
            start_time: sec_start_date.clone(),
            end_time: sec_end_date,
          });
          sec_end_date = sec_start_date.clone();
          index = (index + 1) % senders.len();
        }
      }
    });
    return Ok(res_recv);
  }

  async fn stop(&self) -> SendableErrorResult<()> {
    let msg = ret_on_err!(to_msgpack(&KlineCtrl::Stop));
    ret_on_err!(self.broker.publish("binance.kline.ctrl", &msg[..]).await);
    return Ok(());
  }
}
