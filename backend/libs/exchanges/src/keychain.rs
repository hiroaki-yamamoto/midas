use ::mongodb::{Collection, Database};

use crate::traits::Recorder;

pub struct KeyChain {
  db: Database,
  col: Collection,
}

impl KeyChain {
  fn new(db: Database) -> Self {
    let col = db.collection("apiKeyChains");
    let ret = Self { db, col };
    ret.update_indices(&["exchange"]);
    return ret;
  }
}

impl Recorder for KeyChain {
  fn get_database(&self) -> &Database {
    return self.db;
  }

  fn get_col_name(&self) -> &str {
    return self.col.name();
  }
}
