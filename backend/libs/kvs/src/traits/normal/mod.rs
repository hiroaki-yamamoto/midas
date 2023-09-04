mod base;
mod exist;
mod expiration;
mod get;
mod list_op;
mod lock;
mod remove;
mod set;

pub use self::base::Base;
pub use self::exist::Exist;
pub use self::expiration::Expiration;
pub use self::get::Get;
pub use self::list_op::ListOp;
pub use self::lock::Lock;
pub use self::remove::Remove;
pub use self::set::Set;

use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};

#[async_trait]
pub trait Store<T, V>:
  Base<T, V>
  + Exist<T, V>
  + Expiration<T, V>
  + Get<T, V>
  + ListOp<T, V>
  + Lock<T, V>
  + Remove<T, V>
  + Set<T, V>
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
{
}
