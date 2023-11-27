use ::std::marker::PhantomData;
use ::std::sync::Arc;

use ::futures::stream::{iter, BoxStream, StreamExt};

use ::errors::KVSResult;
use ::kvs::redis::AsyncCommands;
use ::kvs::traits::last_checked::{Get, SetOp};
use ::rpc::exchanges::Exchanges;

pub struct NodeIndexer<T>
where
  T: AsyncCommands + Send + Sync + 'static,
{
  indexer: Arc<dyn SetOp<Commands = T, Value = String> + Send + Sync>,
  exchange_type_kvs: Arc<dyn Get<Commands = T, Value = String> + Send + Sync>,
  _t: PhantomData<T>,
}

impl<T> NodeIndexer<T>
where
  T: AsyncCommands + Send + Sync + 'static,
{
  pub fn new<KVS>(exchange_type_kvs: Arc<KVS>) -> Self
  where
    KVS: Get<Commands = T, Value = String>
      + SetOp<Commands = T, Value = String>
      + Send
      + Sync
      + 'static,
  {
    return Self {
      indexer: exchange_type_kvs.clone(),
      exchange_type_kvs,
      _t: PhantomData,
    };
  }

  fn chname(&self) -> Arc<String> {
    return "node_index".to_string().into();
  }

  /// Index node ids to KVS
  pub async fn index_node(&self, node: String) -> KVSResult<usize> {
    return self.indexer.sadd(self.chname(), node).await;
  }

  /// Unindex node ids to KVS
  pub async fn unindex_node(&self, node: String) -> KVSResult<usize> {
    return self.indexer.srem(self.chname(), node).await;
  }

  pub async fn get_nodes_by_exchange(
    &self,
    exchange: Box<Exchanges>,
  ) -> KVSResult<BoxStream<'_, Arc<String>>> {
    let keys: Vec<String> = self.indexer.smembers(self.chname()).await?;
    let exchange: Exchanges = *exchange.clone();

    let keys = iter(keys)
      .map(move |key| {
        let key: Arc<String> = key.into();
        let exchange = self.exchange_type_kvs.get(key.clone());
        return (key, exchange);
      })
      .filter_map(|(key, exchange)| async {
        let pair = exchange.await.map(|exchange| {
          let exchange: Result<Exchanges, _> = exchange.parse();
          return exchange.map(|e| (key, e)).ok();
        });
        let pair = pair.ok().flatten();
        return pair;
      })
      .filter_map(move |(key, node_exchange)| async move {
        if exchange == node_exchange {
          return Some(key);
        } else {
          return None;
        }
      });
    return Ok(keys.boxed());
  }
}
