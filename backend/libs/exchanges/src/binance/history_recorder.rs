use ::std::time::Duration;

use ::crossbeam::channel::{Receiver, Sender};
use ::mongodb::bson::to_bson;
use ::mongodb::Collection;
use ::slog::Logger;

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
    let col = self.col.clone();
    ::tokio::spawn(async move {
      while let Err(_) = stop.try_recv() {
        if let Ok(kline_reuslt) = value_ch.recv_timeout(Duration::from_nanos(1))
        {
          match kline_reuslt {
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
              ::tokio::task::block_in_place(|| {
                let _ = prog_ch.send(Ok(prog));
              });
              let klines = raw_klines
                .into_iter()
                .filter_map(|item| item.ok())
                .filter_map(|item| to_bson(&item).ok())
                .filter_map(|item| item.as_document().cloned())
                .map(|item| item.clone());
              let _ = col.insert_many(klines, None).await;
            }
          }
        }
      }
      drop(prog_ch);
    });
  }
}
