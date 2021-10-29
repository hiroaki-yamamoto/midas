use ::async_trait::async_trait;
use ::errors::EmptyError;
use ::futures::future::{join3, join_all};
use ::futures::{Stream, StreamExt};
use ::mongodb::bson::oid::ObjectId;
use ::mongodb::bson::{
  doc, from_document, DateTime as MongoDateTime, Document,
};
use ::mongodb::error::Result as MongoResult;
use ::mongodb::{Collection, Database};
use ::nats::Connection as NatsConnection;
use ::rmp_serde::to_vec as to_msgpack;
use ::slog::{crit, error, warn, Logger};
use ::subscribe::PubSub;
use ::tokio::select;
use ::tokio::sync::mpsc;
use ::types::ThreadSafeResult;

use ::rpc::historical::HistChartProg;

use super::entities::{Kline, Klines, TradeTime};
use super::pubsub::{
  HistFetchRespPubSub, HistProgPartPubSub, RecLatestTradeDatePubSub,
};

use base_recorder::Recorder;
use history::HistoryRecorder as HistRecTrait;

#[derive(Debug, Clone)]
pub struct HistoryRecorder {
  col: Collection<Kline>,
  db: Database,
  prog_pubsub: HistProgPartPubSub,
  fetch_resp_pubsub: HistFetchRespPubSub,
  latest_trade_date_pubsub: RecLatestTradeDatePubSub,
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
      prog_pubsub: HistProgPartPubSub::new(broker.clone()),
      fetch_resp_pubsub: HistFetchRespPubSub::new(broker.clone()),
      latest_trade_date_pubsub: RecLatestTradeDatePubSub::new(broker.clone()),
      logger,
    };
    ret
      .update_indices(&["open_time", "close_time", "symbol"])
      .await;
    return ret;
  }

  async fn spawn_record(self, mut kline_recv: mpsc::UnboundedReceiver<Klines>) {
    let col = self.col.clone();
    loop {
      select! {
        Some(klines) = kline_recv.recv() => {
          let _ = col.insert_many(klines, None).await;
        },
        else => {break;}
      }
    }
  }

  async fn get_latest_trade_time(
    &self,
    symbols: String,
  ) -> ThreadSafeResult<TradeTime<MongoDateTime>> {
    let doc = self
      .col
      .aggregate(
        vec![
          doc! { "$match": doc! { "symbol": symbols } },
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
      .await?
      .filter_map(|doc| async { doc.ok() })
      .filter_map(|doc| async {
        from_document::<TradeTime<MongoDateTime>>(doc).ok()
      })
      .boxed()
      .next()
      .await
      .ok_or(EmptyError {
        field: "Trade Date".to_string(),
      })?;
    return Ok(doc);
  }

  async fn spawn_fetch_response(
    &self,
    senders: Vec<mpsc::UnboundedSender<Klines>>,
  ) {
    let value_sub = match self.fetch_resp_pubsub.queue_subscribe("recorder") {
      Err(e) => {
        crit!(
          self.logger,
          "Failed to subscribe the response channel: {}",
          e; "chan_name" => self.fetch_resp_pubsub.get_subject(),
        );
        return;
      }
      Ok(v) => v,
    };
    let mut value_sub = Box::pin(value_sub);
    let senders = senders.clone();
    let mut counter: usize = 0;
    loop {
      select! {
        Some((klines, _)) = value_sub.next() => {
          let _ = self.prog_pubsub.publish(&HistChartProg {
            id: ObjectId::new().to_string(),
            symbol: klines.symbol,
            num_symbols: klines.num_symbols,
            cur_symbol_num: 1,
            num_objects: klines.entire_data_len as i64,
            cur_object_num: 1,
          });
          let _ = senders[counter].send(klines.klines);
          counter = (counter + 1) % senders.len();
        },
        else => {break;}
      }
    }
  }

  async fn spawn_latest_trade_time_request(&self) {
    let me = self.clone();
    let sub = match me.latest_trade_date_pubsub.queue_subscribe("recorder") {
      Err(e) => {
        error!(
          me.logger,
          "Failed to subscribe latest trade time channel: {}", e;
          "fn" => "spawn_latest_trade_time_request"
        );
        return;
      }
      Ok(v) => v,
    };
    let mut sub = sub.boxed();
    loop {
      let logger = self.logger.clone();
      let me = self.clone();
      select! {
        Some((symbol, msg)) = sub.next() => {
          let trade_dates = match me.get_latest_trade_time(symbol).await {
            Err(e) => {
              error!(logger, "Failed to get the latest trade time: {}", e);
              continue;
            },
            Ok(v) => v
          };
          match msg.reply {
            Some(_) => {
              let resp = match to_msgpack(&trade_dates) {
                Err(e) => {
                  error!(logger, "Failed to encode the response message: {}", e);
                  continue;
                },
                Ok(v) => v
              };
              let _ = msg.respond(resp);
            },
            None => {
              warn!(logger, "The request doesn't have reply subject.");
              continue;
            }
          }
        },
        else => {break;}
      }
    }
  }

  pub async fn list(
    &self,
    query: impl Into<Option<Document>>,
  ) -> MongoResult<impl Stream<Item = Kline>> {
    return Ok(
      self
        .col
        .find(query, None)
        .await?
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
      let me = self.clone();
      let (kline_sender, kline_recvr) = mpsc::unbounded_channel();
      senders.push(kline_sender);
      let worker = me.spawn_record(kline_recvr);
      record_workers.push(worker);
    }
    let me = self.clone();
    let resp_thread =
      ::tokio::spawn(async move { me.spawn_fetch_response(senders).await });
    let me = self.clone();
    let latest_trade_time_thread =
      ::tokio::spawn(async move { me.spawn_latest_trade_time_request().await });
    let record_worker_thread = ::tokio::spawn(join_all(record_workers));
    let _ =
      join3(resp_thread, latest_trade_time_thread, record_worker_thread).await;
  }
}
