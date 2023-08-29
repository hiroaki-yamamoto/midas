use ::kvs::kvs;
use ::kvs::redis::Commands;

use ::dlock::Dlock;

kvs!(pub, InitLock, String, "init_lock:{}");
impl<T> Dlock<T> for InitLock<T> where T: Commands + Send + Sync {}
