use ::kvs::kvs;
use ::kvs::redis::Commands;
use ::kvs::SoftExpirationStore;

kvs!(pub, ObserverNodeKVS, String, "observer_node:{}");
kvs!(
  pub,
  ObserverNodeLastCheckKVS,
  u64,
  "observer_node_last_check_epoch:{}"
);

impl<T> SoftExpirationStore<T, String> for ObserverNodeKVS<T> where T: Commands {}
