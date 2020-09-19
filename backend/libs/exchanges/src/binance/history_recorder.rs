use ::mongodb::bson::to_bson;
use ::mongodb::Collection;
use ::tokio::select;
use ::tokio::sync::{broadcast, mpsc};
use ::tokio::task::block_in_place;

use ::rpc::historical::HistChartProg;
use ::types::SendableErrorResult;

use super::entities::KlineResultsWithSymbol;

#[derive(Debug, Clone)]
pub struct HistoryRecorder {
  col: Collection,
}

impl HistoryRecorder {
  pub fn new(col: Collection) -> Self {
    return Self { col };
  }

  pub fn spawn(
    &self,
    mut stop: broadcast::Receiver<()>,
    mut value_ch: mpsc::UnboundedReceiver<
      SendableErrorResult<KlineResultsWithSymbol>,
    >,
    prog_ch: mpsc::UnboundedSender<SendableErrorResult<HistChartProg>>,
  ) {
    let col = self.col.clone();
    ::tokio::spawn(async move {
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
                  let klines = block_in_place(move || {
                    return raw_klines
                      .into_iter()
                      .filter_map(|item| item.ok())
                      .filter_map(|item| to_bson(&item).ok())
                      .filter_map(|item| item.as_document().cloned())
                      .map(|item| item.clone());
                  });
                  let _ = col.insert_many(klines, None).await;
                }
              }
            }
          },
        }
      }
      drop(prog_ch);
    });
  }
}
