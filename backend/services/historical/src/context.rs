use ::std::sync::Arc;

use ::entities::HistoryFetchRequest;
use ::history::entities::FetchStatusChanged;
use ::kvs::redis::aio::MultiplexedConnection;
use ::kvs::traits::symbol::Get;
use ::subscribe::PubSub;
use ::symbols::traits::SymbolReader;

type ProgressKVS =
  Arc<dyn Get<Commands = MultiplexedConnection, Value = i64> + Send + Sync>;

pub struct Context {
  num_obj_kvs_get: ProgressKVS,
  sync_prog_kvs_get: ProgressKVS,
  status: Arc<dyn PubSub<Output = FetchStatusChanged> + Send + Sync>,
  splitter: Arc<dyn PubSub<Output = HistoryFetchRequest> + Send + Sync>,
  symbol_reader: Arc<dyn SymbolReader + Send + Sync>,
}

impl Context {
  pub fn new(
    num_obj_kvs_get: ProgressKVS,
    sync_prog_kvs_get: ProgressKVS,
    status: Arc<dyn PubSub<Output = FetchStatusChanged> + Send + Sync>,
    splitter: Arc<dyn PubSub<Output = HistoryFetchRequest> + Send + Sync>,
    symbol_reader: Arc<dyn SymbolReader + Send + Sync>,
  ) -> Self {
    Self {
      num_obj_kvs_get,
      sync_prog_kvs_get,
      status,
      splitter,
      symbol_reader,
    }
  }
}
