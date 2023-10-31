use ::std::fmt::Debug;

use crate::redis::AsyncCommands as Commands;
use crate::redis::{FromRedisValue, ToRedisArgs};

use super::KVS;
use crate::traits::base::Base;

impl<CMD, Value> Base for KVS<CMD, Value>
where
  Value: FromRedisValue + ToRedisArgs + Debug + Send + Sync,
  CMD: Commands + Debug + Clone + Send + Sync,
{
  type Commands = CMD;

  fn __commands__(&self) -> CMD {
    return self.connection.clone();
  }
}
