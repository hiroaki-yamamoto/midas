use ::std::collections::hash_map::HashMap;

use ::chrono::{DateTime as ChronoDateTime, Utc};
use ::futures::StreamExt;
use ::mongodb::bson::{doc, from_document, to_bson, DateTime as MongoDateTime};
use ::mongodb::Collection;
use ::nats::asynk::Connection as NatsConnection;
use ::rmp_serde::{from_slice as from_msgpack, to_vec as to_msgpack};
use ::slog::{crit, error, Logger};
use ::tokio::select;
use ::tokio::sync::{broadcast, mpsc};
use ::tokio::task::block_in_place;

use ::rpc::historical::HistChartProg;
use ::types::{ret_on_err, SendableErrorResult};

use super::constants::{
  HIST_FETCHER_FETCH_PROG_SUB_NAME, HIST_FETCHER_FETCH_RESP_SUB_NAME,
};
use super::entities::{Klines, KlinesWithInfo, LatestTradeTime};

#[derive(Debug, Clone)]
pub struct HistoryRecorder {
  col: Collection,
  broker: NatsConnection,
  logger: Logger,
  senders: Vec<mpsc::UnboundedSender<Klines>>,
  stop: broadcast::Sender<()>,
}

impl HistoryRecorder {
  pub fn new(
    col: Collection,
    stop_sender: broadcast::Sender<()>,
    logger: Logger,
    broker: NatsConnection,
  ) -> Self {
    let mut ret = Self {
      col,
      senders: vec![],
      stop: stop_sender,
      broker,
      logger,
    };
    for _ in 0..::num_cpus::get() {
      ret.spawn_record();
    }
    return ret;
  }

  fn spawn_record(&mut self) {
    let (sender, mut recver) = mpsc::unbounded_channel::<Klines>();
    let col = self.col.clone();
    let mut stop = self.stop.subscribe();
    ::tokio::spawn(async move {
      loop {
        select! {
          _ = stop.recv() => {break;},
          raw_klines = recver.recv() => {
            let raw_klines = match raw_klines {
              Some(v) => v,
              None => {break;}
            };
            let klines = block_in_place(move || {
              return raw_klines
                .into_iter()
                .filter_map(|item| to_bson(&item).ok())
                .filter_map(|item| item.as_document().cloned())
                .map(|item| item.clone());
            });
            let _ = col.insert_many(klines, None).await;
          },
        }
      }
    });
    self.senders.push(sender);
  }

  pub async fn spawn(
    &self,
    prog_ch: mpsc::UnboundedSender<SendableErrorResult<HistChartProg>>,
  ) {
    let value_sub = match self
      .broker
      .queue_subscribe(HIST_FETCHER_FETCH_RESP_SUB_NAME, "recorder")
      .await
    {
      Err(e) => {
        crit!(
          self.logger,
          "Failed to subscribe the response channel: {}",
          e; "chan_name" => HIST_FETCHER_FETCH_RESP_SUB_NAME,
        );
        return;
      }
      Ok(v) => v,
    }
    .filter_map(|item| async move {
      return from_msgpack::<KlinesWithInfo>(item.data.as_slice()).ok();
    });
    let mut value_sub = Box::pin(value_sub);
    let senders = self.senders.clone();
    let mut stop = self.stop.subscribe();
    let broker = self.broker.clone();
    let logger = self.logger.clone();
    ::tokio::spawn(async move {
      let mut counter: usize = 0;
      loop {
        select! {
          _ = stop.recv() => {break;},
          Some(klines) = value_sub.next() => {
            let prog = HistChartProg {
                symbol: klines.symbol,
                num_symbols: klines.num_symbols,
                cur_symbol_num: 1,
                num_objects: klines.entire_data_len,
                cur_object_num: 1,
              };
            let prog_msg = match to_msgpack(&prog) {
              Err(e) => {
                error!(logger, "Failed to encode the prog msg: {}", e);
                return;
              },
              Ok(v) => v
            };
            let _ = senders[counter].send(klines.klines);
            counter = (counter + 1) % senders.len();
            broker.publish(HIST_FETCHER_FETCH_PROG_SUB_NAME, prog_msg.as_slice()).await;
          },
        }
      }
    });
  }

  pub async fn get_latest_trade_time(
    &self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<HashMap<String, LatestTradeTime<ChronoDateTime<Utc>>>>
  {
    let mut cur = ret_on_err!(
      self
        .col
        .aggregate(
          vec![
            doc! { "$match": doc! { "symbol": doc! { "$in": symbols } } },
            doc! {
              "$group": doc! {
                "_id": "$symbol",
                "open_time": doc! {
                  "$max": "$open_time"
                },
                "close_time": doc! {
                  "$max": "$close_time"
                }
              }
            }
          ],
          None
        )
        .await
    );
    let mut ret = HashMap::new();
    while let Some(doc) = cur.next().await {
      let doc = ret_on_err!(doc);
      let latest: LatestTradeTime<MongoDateTime> =
        ret_on_err!(from_document(doc));
      let latest: LatestTradeTime<ChronoDateTime<Utc>> = latest.into();
      ret.insert(latest.symbol.clone(), latest);
    }
    return Ok(ret);
  }
}
