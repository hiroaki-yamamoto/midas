mod base;
mod expiration;
mod get;
mod list_op;
mod remove;
mod set;

use ::redis::{Commands, FromRedisValue, ToRedisArgs};

pub use self::base::Base;
pub use self::expiration::Expiration;
pub use self::get::Get;
pub use self::list_op::ListOp;
pub use self::remove::Remove;
pub use self::set::Set;

pub trait LastCheckStore<T, V>:
  Base<T> + Expiration<T> + Get<T, V> + ListOp<T, V> + Remove<T> + Set<T, V>
where
  T: Commands + Send,
  for<'async_trait> V: FromRedisValue + ToRedisArgs + Send + 'async_trait,
{
}
