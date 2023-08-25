use ::kvs::kvs;
use ::kvs::redis::{Commands, RedisResult};
use ::kvs::SoftExpirationStore;
use kvs::Store;

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

impl<T> SoftExpirationStore<T, String> for ObserverNodeKVS<T> where T: Commands {}
impl<T> SoftExpirationStore<T, String> for ONEXTypeKVS<T> where T: Commands {}

impl<T> ObserverNodeKVS<T>
where
  T: Commands,
{
  pub fn count_nodes(&self) -> RedisResult<usize> {
    return Ok(
      self
        .lock_commands()
        .scan_match::<_, String>("observer_node:*")?
        .count(),
    );
  }
}
