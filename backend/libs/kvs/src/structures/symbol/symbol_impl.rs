use ::std::fmt::Debug;
use ::std::sync::Arc;

use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::symbol::{ChannelName, Get, Remove, Set};

impl<CMD, Value> ChannelName for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync + Debug,
  CMD: Commands + Clone + Send + Sync + Debug,
{
  fn channel_name(
    &self,
    exchange: Arc<String>,
    symbol: Arc<String>,
  ) -> Arc<String> {
    let channel_name = format!("{}:{}:{}", self.channel_name, exchange, symbol);
    return channel_name.into();
  }
}

impl<CMD, Value> Get for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync + Debug,
  CMD: Commands + Clone + Send + Sync + Debug,
{
  type Value = Value;
}

impl<CMD, Value> Remove for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync + Debug,
  CMD: Commands + Clone + Send + Sync + Debug,
{
}

impl<CMD, Value> Set for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Send + Sync + Debug,
  CMD: Commands + Clone + Send + Sync + Debug,
{
  type Value = Value;
}
