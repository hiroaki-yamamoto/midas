mod service;

use ::futures::FutureExt;
use ::log::{info, warn};
use ::warp::Filter;

use ::access_logger::log;
use ::config::init;
use ::csrf::{CSRFOption, CSRF};
use ::probe::probe;
use ::rpc::rejection_handler::handle_rejection;

use self::service::Service;

#[tokio::main]
async fn main() {
  init(|cfg, mut sig, db, broker, host| async move {
    let svc = Service::new(&db, &broker).await.unwrap();
    let csrf = CSRF::new(CSRFOption::builder());
    let router = csrf
      .protect()
      .and(probe().or(svc.route()))
      .with(log())
      .recover(handle_rejection);

    info!("Opened REST server on {}", host);
    let (_, svr) = ::warp::serve(router)
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
