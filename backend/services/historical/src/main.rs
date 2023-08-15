mod service;

use ::futures::FutureExt;
use ::log::{info, warn};
use ::warp::Filter;

use ::config::init;
use ::csrf::{CSRFOption, CSRF};
use ::rpc::rejection_handler::handle_rejection;

use crate::service::Service;

#[tokio::main]
async fn main() {
  init(|cfg, mut sig, db, broker, host| async move {
    info!("Historical Kline Service");
    let svc = Service::new(&broker, &cfg.redis, &db).await.unwrap();
    let csrf = CSRF::new(CSRFOption::builder());
    let access_log = ::access_logger::log();
    let route = csrf
      .protect()
      .and(svc.route())
      .recover(handle_rejection)
      .with(access_log);

    let host = host.clone();
    info!("Opened REST server on {}", host);
    let (_, ws_svr) = ::warp::serve(route)
      .tls()
      .cert_path(&cfg.tls.cert)
      .key_path(&cfg.tls.prv_key)
      .bind_with_graceful_shutdown(host, async move {
        sig.recv().await;
      });
    let svr = ws_svr.then(|_| async {
      warn!("REST Server is shutting down! Bye! Bye!");
    });
    svr.await;
  })
  .await;
}
