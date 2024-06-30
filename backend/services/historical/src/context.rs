use ::std::sync::Arc;

use ::futures::future::{try_join, TryFutureExt};
use ::futures::stream::{BoxStream, StreamExt};
use ::log::warn;
use ::serde_json::to_string as jsonify;
use ::warp::ws::Message;

use ::history::entities::FetchStatusChanged;
use ::rpc::exchanges::Exchanges;
use ::rpc::progress::Progress;
use ::subscribe::PubSub;
use ::symbols::traits::SymbolReader;

use super::types::ProgressKVS;
use crate::errors::ServiceResult;
use crate::services::{ISocketRequestService, ISocketResponseService};

pub struct Context {
  pub num_obj: ProgressKVS,
  pub sync_prog: ProgressKVS,
  pub status: Arc<dyn PubSub<Output = FetchStatusChanged> + Send + Sync>,
  pub symbol_reader: Arc<dyn SymbolReader + Send + Sync>,
  pub socket_response: Arc<dyn ISocketResponseService + Send + Sync>,
  pub socket_request: Arc<dyn ISocketRequestService + Send + Sync>,
}

impl Context {
  pub fn new(
    num_obj: ProgressKVS,
    sync_prog: ProgressKVS,
    status: Arc<dyn PubSub<Output = FetchStatusChanged> + Send + Sync>,
    symbol_reader: Arc<dyn SymbolReader + Send + Sync>,
    socket_response: Arc<dyn ISocketResponseService + Send + Sync>,
    socket_request: Arc<dyn ISocketRequestService + Send + Sync>,
  ) -> Self {
    Self {
      num_obj,
      sync_prog,
      status,
      symbol_reader,
      socket_response,
      socket_request,
    }
  }

  pub async fn get_init_prog_stream(
    &self,
    exchange: Exchanges,
  ) -> ServiceResult<BoxStream<Message>> {
    let ret = self
      .symbol_reader
      .list_trading()
      .map_ok(move |list_stream| {
        return list_stream
          .map(|symbol| symbol.symbol)
          .filter_map(move |symbol| async move {
            let exchange_name: Arc<String> =
              Arc::new(exchange.to_string().to_lowercase());
            let sym: Arc<String> = Arc::new(symbol);
            let size_cur = try_join(
              self.num_obj.get(exchange_name.clone(), sym.clone()),
              self.sync_prog.get(exchange_name.clone(), sym.clone()),
            )
            .await;
            return match size_cur {
              Ok((size, cur)) => {
                let size_cur_pair = size.zip(cur);
                if let Some((size, cur)) = size_cur_pair {
                  Some((size, cur, sym))
                } else {
                  None
                }
              }
              Err(err) => {
                warn!(
                  error: err = err,
                  exchange: serde = exchange_name,
                  symbol: serde = sym;
                  "Failed to get progress"
                );
                None
              }
            };
          })
          .map(|(size, cur, symbol)| {
            return Progress {
              exchange: Box::new(Exchanges::Binance),
              symbol: symbol.to_string(),
              size,
              cur,
            };
          })
          .filter_map(|prog: Progress| async move {
            return jsonify(&prog).ok();
          })
          .map(|payload: String| {
            return Message::text(payload);
          })
          .boxed();
      })
      .await?;
    return Ok(ret);
  }
}
