mod channel_name;
mod get;
mod incr;
mod remove;
mod set;

use super::normal::Base;

pub use self::channel_name::ChannelName;
pub use self::get::Get;
pub use self::incr::Incr;
pub use self::remove::Remove;
pub use self::set::Set;

use ::redis::{Commands, FromRedisValue, ToRedisArgs};

pub trait SymbolKeyStore<T, V>:
  Base<T> + ChannelName + Get<T, V> + Remove<T> + Set<T, V>
where
  T: Commands + Send,
  for<'a> V: FromRedisValue + ToRedisArgs + Send + 'a,
{
}
