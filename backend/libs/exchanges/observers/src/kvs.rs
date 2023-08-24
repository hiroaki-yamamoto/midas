use ::kvs::kvs;
use ::kvs::redis::Commands;
use ::kvs::SoftExpirationStore;

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
