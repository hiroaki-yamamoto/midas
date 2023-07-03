use ::futures::FutureExt;
use ::log::{info, warn};
use ::warp::reply;
use ::warp::Filter;

use ::access_logger::log;
use ::config::init;
use ::csrf::{CSRFOption, CSRF};
use ::rpc::rejection_handler::handle_rejection;

#[tokio::main]
async fn main() {
  init(|cfg, mut sig, _, _, host| async move {
    let csrf = CSRF::new(CSRFOption::builder());
    let route = ::warp::get()
      .or(::warp::head())
      .or(::warp::options())
      .and(::warp::path("csrf"))
      .map(|_| reply::reply());
    let route = csrf
      .generate_cookie(route)
      .with(log())
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
  .await
}
