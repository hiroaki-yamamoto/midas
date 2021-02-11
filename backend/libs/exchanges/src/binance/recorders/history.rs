use ::std::collections::hash_map::HashMap;

use ::async_trait::async_trait;
use ::futures::future::{join3, join_all};
use ::futures::{Stream, StreamExt};
use ::mongodb::bson::{
  doc, from_document, to_bson, DateTime as MongoDateTime, Document,
};
use ::mongodb::error::Result as MongoResult;
use ::mongodb::{Collection, Database};
use ::nats::asynk::Connection as NatsConnection;
use ::rmp_serde::{from_slice as from_msgpack, to_vec as to_msgpack};
use ::slog::{crit, error, warn, Logger};
use ::tokio::select;
use ::tokio::sync::mpsc;
use ::tokio::task::block_in_place;

use ::rpc::historical::HistChartProg;
use ::types::{GenericResult, ThreadSafeResult};

use super::super::constants::{
  HIST_FETCHER_FETCH_PROG_SUB_NAME, HIST_FETCHER_FETCH_RESP_SUB_NAME,
  HIST_RECORDER_LATEST_TRADE_DATE_SUB_NAME,
};
use super::super::entities::{Kline, Klines, KlinesWithInfo, TradeTime};

use crate::traits::{HistoryRecorder as HistRecTrait, Recorder};

#[derive(Debug, Clone)]
pub struct HistoryRecorder {
  col: Collection,
  db: Database,
  broker: NatsConnection,
  logger: Logger,
}

impl Recorder for HistoryRecorder {
  fn get_database(&self) -> &Database {
    return &self.db;
  }
  fn get_col_name(&self) -> &str {
    return self.col.name();
  }
}

impl HistoryRecorder {
  pub async fn new(
    db: Database,
    logger: Logger,
    broker: NatsConnection,
  ) -> Self {
    let col = db.collection("binance.klines");
    let ret = Self {
      db,
      col,
      broker,
      logger,
    };
    ret
      .update_indices(&["open_time", "close_time", "symbol"])
      .await;
    return ret;
  }

  async fn spawn_record(
    &self,
    mut kline_recv: mpsc::UnboundedReceiver<Klines>,
  ) {
    let col = self.col.clone();
    loop {
      select! {
        Some(raw_klines) = kline_recv.recv() => {
          let klines = block_in_place(move || {
            return raw_klines
              .into_iter()
              .filter_map(|item| to_bson(&item).ok())
              .filter_map(|item| item.as_document().cloned())
              .map(|item| item.clone());
          });
          let _ = col.insert_many(klines, None).await;
        },
        else => {break;}
      }
    }
  }

  async fn get_latest_trade_time(
    &self,
    symbols: Vec<String>,
  ) -> MongoResult<HashMap<String, TradeTime<MongoDateTime>>> {
    let mut cur = self
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
          },
        ],
        None,
      )
      .await?;
    let mut ret = HashMap::new();
    while let Some(doc) = cur.next().await {
      let doc = doc?;
      let latest: TradeTime<MongoDateTime> = from_document(doc)?;
      ret.insert(latest.symbol.clone(), latest);
    }
    return Ok(ret);
  }

  async fn spawn_fetch_response(
    &self,
    senders: Vec<mpsc::UnboundedSender<Klines>>,
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
    let senders = senders.clone();
    let broker = self.broker.clone();
    let logger = self.logger.clone();
    let mut counter: usize = 0;
    loop {
      select! {
        Some(klines) = value_sub.next() => {
          let prog = HistChartProg {
              symbol: klines.symbol,
              num_symbols: klines.num_symbols as u64,
              cur_symbol_num: 1,
              num_objects: klines.entire_data_len as u64,
              cur_object_num: 1,
            };
          let prog_msg = match to_msgpack(&prog) {
            Err(e) => {
              error!(logger, "Failed to encode the prog msg: {}", e);
              return;
            },
            Ok(v) => v
          };
          let _ = broker.publish(
            HIST_FETCHER_FETCH_PROG_SUB_NAME, prog_msg.as_slice()
          ).await;
          let _ = senders[counter].send(klines.klines);
          counter = (counter + 1) % senders.len();
        },
        else => {break;}
      }
    }
  }

  async fn spawn_latest_trade_time_request(&self) {
    let me = self.clone();
    let mut sub = Box::pin(
      match me
        .broker
        .queue_subscribe(HIST_RECORDER_LATEST_TRADE_DATE_SUB_NAME, "recorder")
        .await
      {
        Err(e) => {
          error!(
            me.logger,
            "Failed to subscribe latest trade time channel: {}", e;
            "fn" => "spawn_latest_trade_time_request"
          );
          return;
        }
        Ok(v) => v,
      }
      .map(|msg| {
        return (from_msgpack::<Vec<String>>(&msg.data[..]), msg);
      })
      .filter_map(|(item, msg)| async move { Some((item.ok()?, msg)) }),
    );
    loop {
      select! {
        Some((symbols, msg)) = sub.next() => {
          let trade_dates = match me.get_latest_trade_time(symbols).await {
            Err(e) => {
              error!(me.logger, "Failed to get the latest trade time: {}", e);
              continue;
            },
            Ok(v) => v
          };
          match msg.reply {
            Some(_) => {
              let resp = match to_msgpack(&trade_dates) {
                Err(e) => {
                  error!(me.logger, "Failed to encode the response message: {}", e);
                  continue;
                },
                Ok(v) => v
              };
              let _ = msg.respond(resp).await;
            },
            None => {
              warn!(me.logger, "The request doesn't have reply subject.");
              continue;
            }
          }
        },
        else => {break;}
      }
    }
  }

  pub(crate) async fn list(
    &self,
    query: impl Into<Option<Document>>,
  ) -> MongoResult<impl Stream<Item = Kline>> {
    return Ok(
      self
        .col
        .find(query, None)
        .await?
        .filter_map(|item| async { item.ok() })
        .map(|item| from_document::<Kline>(item))
        .filter_map(|item| async { item.ok() }),
    );
  }
}

#[async_trait]
impl HistRecTrait for HistoryRecorder {
  async fn spawn(&self) {
    let mut record_workers = vec![];
    let mut senders = vec![];
    for _ in 0..num_cpus::get() {
      let (kline_sender, kline_recvr) = mpsc::unbounded_channel();
      senders.push(kline_sender);
      let worker = self.spawn_record(kline_recvr);
      record_workers.push(worker);
    }
    join3(
      self.spawn_fetch_response(senders),
      self.spawn_latest_trade_time_request(),
      join_all(record_workers),
    )
    .await;
  }
}
