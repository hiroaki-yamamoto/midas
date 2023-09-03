mod base;
mod exist;
mod expiration;
mod get;
mod list_op;
mod lock;
mod remove;
mod set;

pub use self::base::Base as NormalStoreBase;
pub use self::exist::Exist as NormalStoreExist;
pub use self::expiration::Expiration as NormalStoreExpiration;
pub use self::get::Get as NormalStoreGet;
pub use self::list_op::ListOp as NormalStoreListOp;
pub use self::lock::Lock as NormalStoreLock;
pub use self::remove::Remove as NormalStoreRemove;
pub use self::set::Set as NormalStoreSet;

use ::async_trait::async_trait;
use ::redis::{Commands, FromRedisValue, ToRedisArgs};

#[async_trait]
pub trait Store<T, V>:
  NormalStoreBase<T, V>
  + NormalStoreExist<T, V>
  + NormalStoreExpiration<T, V>
  + NormalStoreGet<T, V>
  + NormalStoreListOp<T, V>
  + NormalStoreLock<T, V>
  + NormalStoreRemove<T, V>
  + NormalStoreSet<T, V>
where
  T: Commands + Send,
  V: FromRedisValue + ToRedisArgs + Send,
{
}
