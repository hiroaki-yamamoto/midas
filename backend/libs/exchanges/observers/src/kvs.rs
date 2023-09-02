use ::futures::stream::{iter, BoxStream, StreamExt};

use ::kvs::kvs;
use ::kvs::redis::{Commands, RedisResult};
use ::kvs::{SoftExpirationStore, Store};

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

impl<T> SoftExpirationStore<T, String> for ObserverNodeKVS<T> where
  T: Commands + Send + Sync
{
}
impl<T> SoftExpirationStore<T, String> for ONEXTypeKVS<T> where
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
