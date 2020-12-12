use ::mongodb::{Collection, Database};

use crate::traits::Recorder as RecorderTrait;

pub struct OrderRecorder {
  col: Collection,
  db: Database,
}

impl OrderRecorder {
  pub async fn new(db: Database) -> Self {
    let ret = Self {
      col: (&db).collection("binance.orders"),
      db,
    };
    ret.update_indices(&["botId", "symbol"]).await;
    return ret;
  }
}

impl RecorderTrait for OrderRecorder {
  fn get_database(&self) -> &Database {
    return &self.db;
  }
  fn get_col_name(&self) -> &str {
    return &self.col.name();
  }
}
