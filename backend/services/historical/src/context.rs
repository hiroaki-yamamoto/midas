use ::std::sync::Arc;

use ::futures::future::{try_join, TryFutureExt};
use ::futures::stream::{BoxStream, StreamExt};
use ::http::StatusCode;
use ::log::{as_error, warn};
use ::serde_json::to_string as jsonify;
use ::warp::reject::{custom as cus_rej, Rejection};
use ::warp::ws::Message;

use ::entities::HistoryFetchRequest;
use ::history::entities::FetchStatusChanged;
use ::kvs::redis::aio::MultiplexedConnection;
use ::kvs::traits::symbol::Get;
use ::rpc::exchanges::Exchanges;
use ::rpc::progress::Progress;
use ::rpc::status::Status;
use ::subscribe::PubSub;
use ::symbols::traits::SymbolReader;

type ProgressKVS =
  Arc<dyn Get<Commands = MultiplexedConnection, Value = i64> + Send + Sync>;

pub struct Context {
  num_obj: ProgressKVS,
  sync_prog: ProgressKVS,
  status: Arc<dyn PubSub<Output = FetchStatusChanged> + Send + Sync>,
  splitter: Arc<dyn PubSub<Output = HistoryFetchRequest> + Send + Sync>,
  symbol_reader: Arc<dyn SymbolReader + Send + Sync>,
}

impl Context {
  pub fn new(
    num_obj: ProgressKVS,
    sync_prog: ProgressKVS,
    status: Arc<dyn PubSub<Output = FetchStatusChanged> + Send + Sync>,
    splitter: Arc<dyn PubSub<Output = HistoryFetchRequest> + Send + Sync>,
    symbol_reader: Arc<dyn SymbolReader + Send + Sync>,
  ) -> Self {
    Self {
      num_obj,
      sync_prog,
      status,
      splitter,
      symbol_reader,
    }
  }

  pub async fn get_init_prog_stream(
    &self,
    exchange: Exchanges,
  ) -> Result<BoxStream<Message>, Rejection> {
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
              Ok((size, cur)) => Some((size, cur, sym)),
              Err(err) => {
                warn!(error = as_error!(err); "Failed to get progress");
                None
              }
            };
          })
          .map(|(size, cur, symbol)| {
            return Progress {
              exchange: Box::new(Exchanges::Binance),
              symbol: symbol.as_ref().clone(),
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
      .map_err(|err| {
        return cus_rej(Status::new(
          StatusCode::SERVICE_UNAVAILABLE,
          &format!("(DB, Symbol): {}", err),
        ));
      })
      .await?;
    return Ok(ret);
  }
}
