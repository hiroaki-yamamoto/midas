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

pub trait SoftExpirationStore<T, V>:
  Base<T, V>
  + Expiration<T, V>
  + Get<T, V>
  + ListOp<T, V>
  + Remove<T, V>
  + Set<T, V>
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
{
}
