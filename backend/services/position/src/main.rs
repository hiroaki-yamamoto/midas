mod context;
mod routing;

use ::std::sync::Arc;

use ::futures::FutureExt;
use ::log::{info, warn};
use ::warp::Filter;

use ::access_logger::log;
use ::config::init;
use ::csrf::{CSRFOption, CSRF};
use ::warp_utils::handle_rejection;

use ::rpc::exchanges::Exchanges;

use crate::context::Context;

#[tokio::main]
async fn main() {
  init(|cfg, mut sig, db, _, host| async move {
    let access_logger = log();
    let ctx = Context::new(Exchanges::Binance, db).await;

    let csrf = CSRF::new(CSRFOption::builder());
    let route = csrf
      .protect()
      .and(routing::construct(Arc::new(ctx)))
      .with(access_logger)
      .recover(handle_rejection);
    info!("Opened REST server on {}", host);
    let (_, svr) = ::warp::serve(route)
      .tls()
      .cert_path(&cfg.tls.cert)
      .key_path(&cfg.tls.prv_key)
      .bind_with_graceful_shutdown(host, async move {
        sig.recv().await;
      });
    let svr = svr.then(|_| async {
      warn!("REST Server is shutting down! Bye! Bye!");
    });
    svr.await;
  })
  .await;
}
