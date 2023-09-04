mod base;
mod channel_name;
mod exist;
mod expiration;
mod get;
mod list_op;
mod lock;
mod remove;
mod set;

pub use self::base::Base;
pub use self::channel_name::ChannelName;
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
  Base<T>
  + Exist<T>
  + Expiration<T>
  + Get<T, V>
  + ListOp<T, V>
  + Lock<T>
  + Remove<T>
  + Set<T, V>
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
{
}
