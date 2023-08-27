use ::config::Database;
use ::kvs::redis::Commands;
use ::kvs::Connection as KVSConnection;

pub struct SyncHandler<T>
where
  T: Commands,
{
  db: Database,
  cmd: KVSConnection<T>,
}

impl<T> SyncHandler<T>
where
  T: Commands,
{
  pub fn new(db: &Database, cmd: KVSConnection<T>) -> Self {
    return Self {
      db: db.clone(),
      cmd,
    };
  }

  pub fn handle(&self) {}
}
