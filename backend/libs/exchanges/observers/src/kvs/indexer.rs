use ::std::marker::PhantomData;
use ::std::sync::Arc;

use ::futures::stream::{iter, BoxStream, StreamExt};

use ::errors::KVSResult;
use ::kvs::redis::AsyncCommands;
use ::kvs::traits::last_checked::{Get, SetOp};
use ::rpc::entities::Exchanges;

pub struct NodeIndexer<T, ExchangeTypeKVS>
where
  T: AsyncCommands + Send + Sync,
  ExchangeTypeKVS: Get<T, String> + SetOp<T, String> + Send + Sync,
{
  exchange_type_kvs: ExchangeTypeKVS,
  _t: PhantomData<T>,
}

impl<T, ExchangeTypeKVS> NodeIndexer<T, ExchangeTypeKVS>
where
  T: AsyncCommands + Send + Sync,
  ExchangeTypeKVS: Get<T, String> + SetOp<T, String> + Send + Sync,
{
  pub fn new(exchange_type_kvs: ExchangeTypeKVS) -> Self {
    return Self {
      exchange_type_kvs,
      _t: PhantomData,
    };
  }

  fn chname(&self) -> Arc<String> {
    return "node_index".to_string().into();
  }

  /// Index node ids to KVS
  pub async fn index_node(&self, node: String) -> KVSResult<usize> {
    return self.exchange_type_kvs.sadd(self.chname(), node).await;
  }

  /// Unindex node ids to KVS
  pub async fn unindex_node(&self, node: String) -> KVSResult<usize> {
    return self.exchange_type_kvs.srem(self.chname(), node).await;
  }

  pub async fn get_nodes_by_exchange(
    &self,
    exchange: Exchanges,
  ) -> KVSResult<BoxStream<'_, Arc<String>>> {
    let keys: Vec<String> =
      self.exchange_type_kvs.smembers(self.chname()).await?;

    let keys = iter(keys)
      .map(move |key| {
        let key: Arc<String> = key.into();
        let exchange = self.exchange_type_kvs.get(key.clone());
        return (key, exchange);
      })
      .filter_map(|(key, exchange)| async {
        let pair = exchange
          .await
          .map(|exchange| (key, Exchanges::from_str_name(&exchange)));
        return pair.ok();
      })
      .filter_map(|(key, node_exchange)| async move {
        return node_exchange.map(|node_exchange| (key, node_exchange));
      })
      .filter_map(move |(key, node_exchange)| async move {
        if node_exchange == exchange {
          return Some(key);
        } else {
          return None;
        }
      });
    return Ok(keys.boxed());
  }
}
