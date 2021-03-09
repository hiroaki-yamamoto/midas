use ::mongodb::{Collection, Database};
use ::nats::asynk::Connection as NatsCon;
use ::ring::hmac;
use ::slog::Logger;

use ::binance_notification::{UserStream, UserStreamTrait};
use ::errors::ObjectNotFound;
use ::executor::Executor as ExecutorTrait;
use ::keychain::KeyChain;
use ::rpc::entities::Exchanges;
use ::sign::Sign;

pub struct Executor {
  keychain: KeyChain,
  broker: NatsCon,
  db: Database,
  log: Logger,
  uds: UserStream,
  orders: Collection,
  positions: Collection,
}

impl Executor {
  pub async fn new(broker: NatsCon, db: Database, log: Logger) -> Self {
    let keychain = KeyChain::new(broker.clone(), db.clone()).await;
    let orders = db.collection("binance.orders");
    let positions = db.collection("binance.positions");
    let uds = UserStream::new(broker.clone(), log.clone());
    return Self {
      broker,
      keychain,
      db,
      log,
      uds,
      orders,
      positions,
    };
  }
}

// impl ExecutorTrait for Executor {
//   async fn open(
//     &mut self,
//     api_key_id: ObjectId,
//   ) -> GenericResult<LocalBoxStream<'_, GenericResult<BookTicker>>> {
//     let api_keypair = self.keychain.get(Exchanges::Binance, api_key_id).await?;
//     if api_keypair.is_none() {
//       return Err(ObjectNotFound::new("API_KEYPAIR"));
//     }
//   }
// }

impl Sign for Executor {
  fn get_secret_key(&self, prv_key: String) -> hmac::Key {
    return hmac::Key::new(hmac::HMAC_SHA256, prv_key.as_bytes());
  }
}
