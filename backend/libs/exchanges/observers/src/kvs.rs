use ::futures::stream::{iter, BoxStream, StreamExt};

use ::kvs::kvs;
use ::kvs::redis::{Commands, RedisResult};
use ::kvs::traits::last_checked::LastCheckStore;
use ::rpc::entities::Exchanges;

kvs!(pub, ObserverNodeKVS, String, "observer_node:{}");
kvs!(pub, ONEXTypeKVS, String, "observer_node_exchange_type:{}");
kvs!(
  pub,
  ObserverNodeLastCheckKVS,
  u64,
  "observer_node_last_check_epoch:{}"
);
kvs!(
  pub,
  ONEXTypeLastCheckedKVS,
  u64,
  "observer_node_exchange_type_last_checked_epoch:{}"
);

impl<T> LastCheckStore<T, String> for ObserverNodeKVS<T> where
  T: Commands + Send + Sync
{
}

impl<T> ONEXTypeKVS<T>
where
  T: Commands + Send + Sync,
{
  pub fn get_nodes_by_exchange(
    &self,
    exchange: Exchanges,
  ) -> RedisResult<BoxStream<'_, String>> {
    let keys: Vec<String> = self
      .lock_commands()
      .scan_match("observer_node_exchange_type:*")?
      .collect();
    let keys = keys
      .into_iter()
      .map(|key| {
        let mut cmd = self.lock_commands();
        return (key.clone(), cmd.get::<_, String>(key));
      })
      .filter_map(|(key, exchange)| {
        let pair =
          exchange.map(|exchange| (key, Exchanges::from_str_name(&exchange)));
        return pair.ok();
      })
      .filter_map(|(key, node_exchange)| {
        return node_exchange.map(|node_exchange| (key, node_exchange));
      })
      .filter_map(move |(key, node_exchange)| {
        if node_exchange == exchange {
          return Some(key);
        } else {
          return None;
        }
      });
    return Ok(iter(keys).boxed());
  }
}

impl<T> LastCheckStore<T, String> for ONEXTypeKVS<T> where
  T: Commands + Send + Sync
{
}

impl<T> ObserverNodeKVS<T>
where
  T: Commands + Send + Sync,
{
  pub fn get_node_names(&self) -> RedisResult<Vec<String>> {
    return Ok(
      self
        .lock_commands()
        .scan_match("observer_node:*")?
        .collect(),
    );
  }

  pub fn get_handling_symbols(&self) -> RedisResult<BoxStream<String>> {
    let nodes = self.get_node_names()?;
    let mut handling_symbols: Vec<String> = vec![];
    for node in nodes {
      let mut symbols: Vec<String> =
        self.lock_commands().lrange(node, 0, -1)?;
      handling_symbols.append(&mut symbols);
    }
    return Ok(iter(handling_symbols).boxed());
  }

  pub fn count_nodes(&self) -> RedisResult<usize> {
    return Ok(
      self
        .lock_commands()
        .scan_match::<_, String>("observer_node:*")?
        .count(),
    );
  }
}
