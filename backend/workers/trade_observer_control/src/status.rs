use ::num_derive::FromPrimitive;
use ::num_traits::FromPrimitive;

use ::kvs::redis::{FromRedisValue, ToRedisArgs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum Status {
  Init = 0,
  Ready = 1,
}

impl ToRedisArgs for Status {
  fn write_redis_args<W>(&self, out: &mut W)
  where
    W: ?Sized + kvs::redis::RedisWrite,
  {
    return (*self as u8).write_redis_args(out);
  }
}

impl FromRedisValue for Status {
  fn from_redis_value(v: &kvs::redis::Value) -> kvs::redis::RedisResult<Self> {
    let value = FromPrimitive::from_u8(kvs::redis::from_redis_value(v)?)
      .ok_or(kvs::redis::RedisError::from((
        kvs::redis::ErrorKind::TypeError,
        "Invalid status value",
      )))?;
    return Ok(value);
  }
}
