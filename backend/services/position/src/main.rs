mod context;

use ::access_logger::log;
use ::config::init;
use ::csrf::{CSRFOption, CSRF};

use ::rpc::exchanges::Exchanges;

use crate::context::Context;

#[tokio::main]
async fn main() {
  init(|cfg, sig, db, _, host| async move {
    let access_logger = log();

    #[cfg(debug_assertions)]
    let ctx = Context::new(true, Exchanges::Binance, db).await;
    #[cfg(not(debug_assertions))]
    let ctx = Context::new(Exchanges::Binance, db).await;

    let csrf = CSRF::new(CSRFOption::builder());
  })
  .await;
}
