use ::mongodb::{Database, Collection};

use crate::KeyChain;

pub struct Executor {
  keychain: KeyChain,
  db: Database,
  orders: Collection,
  positions: Collection,
}

impl Executor {
  pub async fn new(db: Database) -> Self {
    let keychain = KeyChain::new(db.clone()).await;
    let orders = db.collection("binance.orders");
    let positions = db.collection("binance.positions");
    return Self{ keychain, db, orders, positions };
  }
}
