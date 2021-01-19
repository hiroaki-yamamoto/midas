use ::mongodb::{Collection, Database};
use ::ring::hmac;

use crate::traits::Sign;
use crate::{traits::Executor as ExecutorTrait, KeyChain};

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
    return Self {
      keychain,
      db,
      orders,
      positions,
    };
  }
}

impl Sign for Executor {
  fn get_secret_key(&self, prv_key: String) -> hmac::Key {
    return hmac::Key::new(hmac::HMAC_SHA256, prv_key.as_bytes());
  }
}
