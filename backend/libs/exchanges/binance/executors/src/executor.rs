use ::mongodb::{Collection, Database};
use ::nats::asynk::Connection as NatsCon;
use ::ring::hmac;

use ::keychain::KeyChain;
// use ::executor::Executor as ExecutorTrait;
use ::sign::Sign;

pub struct Executor {
  keychain: KeyChain,
  broker: NatsCon,
  db: Database,
  orders: Collection,
  positions: Collection,
}

impl Executor {
  pub async fn new(broker: NatsCon, db: Database) -> Self {
    let keychain = KeyChain::new(broker.clone(), db.clone()).await;
    let orders = db.collection("binance.orders");
    let positions = db.collection("binance.positions");
    return Self {
      broker,
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
