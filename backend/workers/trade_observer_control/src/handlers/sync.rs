use ::config::Database;

pub struct SyncHandler {
  db: Database,
}

impl SyncHandler {
  pub fn new(db: &Database) -> Self {
    return Self { db: db.clone() };
  }

  pub fn handle(&self) {}
}
