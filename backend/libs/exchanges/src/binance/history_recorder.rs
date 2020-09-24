use ::chrono::{
  DateTime as ChronoDateTime, NaiveDateTime as ChronoNaiveDatetime, Utc,
};
use ::futures::StreamExt;
use ::mongodb::bson::{doc, to_bson, Bson};
use ::mongodb::options::FindOptions;
use ::mongodb::Collection;
use ::tokio::select;
use ::tokio::sync::{broadcast, mpsc};
use ::tokio::task::block_in_place;

use ::rpc::historical::HistChartProg;
use ::types::{ret_on_err, SendableErrorResult};

use crate::errors::FirstTradeDateNotFound;

use super::entities::{KlineResults, KlineResultsWithSymbol};

#[derive(Debug, Clone)]
pub struct HistoryRecorder {
  col: Collection,
  senders: Vec<mpsc::UnboundedSender<KlineResults>>,
  stop: broadcast::Sender<()>,
}

impl HistoryRecorder {
  pub fn new(col: Collection, stop_sender: broadcast::Sender<()>) -> Self {
    let mut ret = Self {
      col,
      senders: vec![],
      stop: stop_sender,
    };
    for _ in 0..::num_cpus::get() {
      ret.spawn_record();
    }
    return ret;
  }

  fn spawn_record(&mut self) {
    let (sender, mut recver) = mpsc::unbounded_channel::<KlineResults>();
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
                .filter_map(|item| item.ok())
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

  pub fn spawn(
    &self,
    mut value_ch: mpsc::UnboundedReceiver<
      SendableErrorResult<KlineResultsWithSymbol>,
    >,
    prog_ch: mpsc::UnboundedSender<SendableErrorResult<HistChartProg>>,
  ) {
    let senders = self.senders.clone();
    let mut stop = self.stop.subscribe();
    ::tokio::spawn(async move {
      let mut counter: usize = 0;
      loop {
        select! {
          _ = stop.recv() => {break;},
          result = value_ch.recv() => {
            if let Some(kline_result) = result {
              match kline_result {
                Err(err) => {
                  let _ = prog_ch.send(Err(err));
                  continue;
                }
                Ok(ok) => {
                  let raw_klines = ok.klines;
                  let prog = HistChartProg {
                    symbol: ok.symbol,
                    num_symbols: ok.num_symbols,
                    cur_symbol_num: 1,
                    num_objects: ok.entire_data_len,
                    cur_object_num: 1,
                  };
                  let _ = prog_ch.send(Ok(prog));
                  let _ = senders[counter].send(raw_klines);
                  counter = (counter + 1) % senders.len()
                }
              }
            }
          },
        }
      }
      drop(prog_ch);
    });
  }

  pub async fn get_latest_trade_open_time(
    &self,
    symbol: &String,
  ) -> SendableErrorResult<ChronoDateTime<Utc>> {
    let mut cur = ret_on_err!(
      self
        .col
        .find(
          doc! {"symbol": symbol},
          FindOptions::builder()
            .sort(doc! {
              "open_time": -1
            })
            .limit(1)
            .build(),
        )
        .await
    );
    if let Some(first) = cur.next().await {
      let first = ret_on_err!(first);
      if let Some(date) = first.get("open_time").and_then(Bson::as_datetime) {
        let ts_sec = date.timestamp();
        let ts_nano = date.timestamp_subsec_nanos();
        let date: ChronoDateTime<Utc> = ChronoDateTime::from_utc(
          ChronoNaiveDatetime::from_timestamp(ts_sec, ts_nano),
          Utc,
        );
        return Ok(date);
      }
    }
    return Err(Box::new(FirstTradeDateNotFound {}));
  }
}
