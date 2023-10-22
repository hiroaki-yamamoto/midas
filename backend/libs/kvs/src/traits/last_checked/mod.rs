mod base;
mod expiration;
mod find_before;
mod get;
mod list_op;
mod remove;
mod set;

use crate::redis::{AsyncCommands as Commands, FromRedisValue, ToRedisArgs};

pub use self::base::Base;
pub use self::expiration::Expiration;
pub use self::find_before::FindBefore;
pub use self::get::Get;
pub use self::list_op::ListOp;
pub use self::remove::Remove;
pub use self::set::Set;

pub trait LastCheckedKVS<T, V>:
  Get<T, V>
  + Set<T, V>
  + Remove<T>
  + ListOp<T, V>
  + FindBefore<T>
  + Expiration<T>
  + Send
  + Sync
where
  T: Commands + Send + Sync,
  for<'a> V: FromRedisValue + ToRedisArgs + Send + Sync + 'a,
{
}
