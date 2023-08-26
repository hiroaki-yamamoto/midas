use ::kvs::kvs;

use crate::status::Status;

kvs!(
  pub,
  InitStatusKVS,
  Status,
  "trade_observer_control:status{}"
);
