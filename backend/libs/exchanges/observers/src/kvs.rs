use ::kvs::exchange_kvs;

exchange_kvs!(pub, ObserverNodeKVS, String, "observer_node:{}");
exchange_kvs!(
  pub,
  ObserverNodeLastCheckKVS,
  u64,
  "observer_node_last_check_epoch:{}"
);
