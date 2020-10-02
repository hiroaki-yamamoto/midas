use ::std::collections::hash_map::HashMap;

use ::async_trait::async_trait;
use ::futures::future::join;
use ::futures::StreamExt;
use ::mongodb::bson::{doc, from_document, to_bson, DateTime as MongoDateTime};
use ::mongodb::{Collection, Database};
use ::nats::asynk::Connection as NatsConnection;
use ::rmp_serde::{from_slice as from_msgpack, to_vec as to_msgpack};
use ::slog::{crit, error, warn, Logger};
use ::tokio::select;
use ::tokio::sync::mpsc;
use ::tokio::task::block_in_place;

use ::rpc::historical::HistChartProg;
use ::types::{ret_on_err, SendableErrorResult};

use super::constants::{
  HIST_FETCHER_FETCH_PROG_SUB_NAME, HIST_FETCHER_FETCH_RESP_SUB_NAME,
  HIST_RECORDER_LATEST_TRADE_DATE_SUB_NAME,
};
use super::entities::{Klines, KlinesWithInfo, LatestTradeTime};

use crate::traits::HistoryRecorder as HistRecTrait;

#[derive(Debug, Clone)]
pub struct HistoryRecorder {
  col: Collection,
  broker: NatsConnection,
  logger: Logger,
  senders: Vec<mpsc::UnboundedSender<Klines>>,
}

impl HistoryRecorder {
  pub fn new(db: Database, logger: Logger, broker: NatsConnection) -> Self {
    let mut ret = Self {
      col: db.collection("binance.klines"),
      senders: vec![],
      broker,
      logger,
    };
    for _ in 0..num_cpus::get() {
      ret.spawn_record();
    }
    return ret;
  }

  fn spawn_record(&mut self) {
    let (sender, mut recver) = mpsc::unbounded_channel::<Klines>();
    let col = self.col.clone();
    ::tokio::spawn(async move {
      loop {
        select! {
          Some(raw_klines) = recver.recv() => {
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
    });
    self.senders.push(sender);
  }

  async fn get_latest_trade_time(
    &self,
    symbols: Vec<String>,
  ) -> SendableErrorResult<HashMap<String, LatestTradeTime<MongoDateTime>>> {
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
      ret.insert(latest.symbol.clone(), latest);
    }
    return Ok(ret);
  }

  async fn spawn_fetch_response(&self) {
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
    let broker = self.broker.clone();
    let logger = self.logger.clone();
    let mut counter: usize = 0;
    loop {
      select! {
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
          let _ = broker.publish(
            HIST_FETCHER_FETCH_PROG_SUB_NAME, prog_msg.as_slice()
          ).await;
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
}

#[async_trait]
impl HistRecTrait for HistoryRecorder {
  async fn spawn(&self) {
    join(
      self.spawn_fetch_response(),
      self.spawn_latest_trade_time_request(),
    )
    .await;
  }
}
