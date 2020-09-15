use ::crossbeam::channel::{Receiver, Sender};
use ::mongodb::bson::{to_bson, Document};
use ::mongodb::Collection;
use ::slog::{error, Logger};

use ::rpc::historical::HistChartProg;
use ::types::SendableErrorResult;

use super::entities::KlineResultsWithSymbol;

#[derive(Debug, Clone)]
pub struct HistoryRecorder {
  col: Collection,
  log: Logger,
}

impl HistoryRecorder {
  pub fn new(col: Collection, log: Logger) -> Self {
    return Self { col, log };
  }

  pub fn spawn(
    &self,
    stop: Receiver<()>,
    value_ch: Receiver<SendableErrorResult<KlineResultsWithSymbol>>,
    prog_ch: Sender<SendableErrorResult<HistChartProg>>,
  ) {
    let me = self.clone();
    ::tokio::spawn(async move {
      while let Err(_) = stop.try_recv() {
        match value_ch.recv() {
          Err(err) => {
            error!(
              me.log,
              "Got an error while receiving Kline Value. error: {}", err
            );
            continue;
          }
          Ok(kline_reuslt) => match kline_reuslt {
            Err(err) => {
              let _ = prog_ch.send(Err(err));
              continue;
            }
            Ok(ok) => {
              let raw_klines = ok.klines;
              let klines: Vec<Document> = raw_klines
                .into_iter()
                .filter_map(|item| item.ok())
                .filter_map(|item| to_bson(&item).ok())
                .filter_map(|item| item.as_document().cloned())
                .map(|item| item.clone())
                .collect();
              let db_insert = me.col.insert_many(klines, None);
              let _ = prog_ch.send(Ok(HistChartProg {
                symbol: ok.symbol,
                num_symbols: ok.num_symbols,
                cur_symbol_num: 1,
                num_objects: ok.entire_data_len,
                cur_object_num: 1,
              }));
              let _ = db_insert.await;
            }
          },
        }
      }
    });
  }
}
