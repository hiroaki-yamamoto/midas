use ::kvs::kvs;
use ::kvs::redis::Commands;
use ::kvs::traits::StoreLock;

kvs!(pub, InitLock, String, "init_lock:{}");

impl<S> StoreLock<S, String> for InitLock<S> where S: Commands + Send + Sync {}
