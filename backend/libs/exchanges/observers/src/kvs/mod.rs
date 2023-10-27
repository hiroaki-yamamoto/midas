mod filter;

use ::std::marker::PhantomData;

use ::futures::stream::{iter, BoxStream, StreamExt};

use ::errors::KVSResult;
use ::kvs::redis::AsyncCommands;
use ::kvs::traits::last_checked::{Get, SetOp};
use ::kvs::{LastCheckedKVSBuilder, NormalKVSBuilder};
use ::rpc::entities::Exchanges;

pub use self::filter::NodeFilter;

pub const NODE_KVS_BUILDER: LastCheckedKVSBuilder<String> =
  LastCheckedKVSBuilder::new("observer_node");

pub const NODE_EXCHANGE_TYPE_KVS_BUILDER: LastCheckedKVSBuilder<String> =
  LastCheckedKVSBuilder::new("observer_node_exchange_type");

pub const INIT_LOCK_BUILDER: NormalKVSBuilder<String> =
  NormalKVSBuilder::<String>::new("init_lock");

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

  /// Index node ids to KVS
  pub async fn index_node(&self, node: String) -> KVSResult<usize> {
    return self.exchange_type_kvs.sadd("node_index", node).await;
  }

  /// Unindex node ids to KVS
  pub async fn unindex_node(&self, node: String) -> KVSResult<usize> {
    return self.exchange_type_kvs.srem("node_index", node).await;
  }

  pub async fn get_nodes_by_exchange(
    &self,
    exchange: Exchanges,
  ) -> KVSResult<BoxStream<'_, String>> {
    let keys: Vec<String> =
      self.exchange_type_kvs.smembers("node_index").await?;

    let keys = iter(keys)
      .map(move |key| {
        let exchange_key = key.clone();
        let exchange = self.exchange_type_kvs.get(&exchange_key);
        return (exchange_key, exchange);
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
