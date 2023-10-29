use ::errors::ObserverResult;
use ::kvs::{LastCheckedKVSBuilder, NormalKVSBuilder};

pub const NODE_KVS_BUILDER: LastCheckedKVSBuilder<String> =
  LastCheckedKVSBuilder::new("observer_node");

pub const NODE_EXCHANGE_TYPE_KVS_BUILDER: LastCheckedKVSBuilder<String> =
  LastCheckedKVSBuilder::new("observer_node_exchange_type");

pub const INIT_LOCK_BUILDER: NormalKVSBuilder<String, ObserverResult<()>> =
  NormalKVSBuilder::new("init_lock");
