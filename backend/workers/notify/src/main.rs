use ::futures::future::{select, Either};

use ::notification::binance;
use ::notification::traits::UserStream as UserStreamTrait;

#[::tokio::main]
async fn main() {
  ::config::init(|_, mut sig, _, broker, _| async move {
    let binance = binance::UserStream::new(&broker).await.unwrap();
    let sig = Box::pin(sig.recv());
    let jobs = binance.start();
    match select(jobs, sig).await {
      Either::Left((v, _)) => v,
      Either::Right(_) => Ok(()),
    }
    .unwrap();
  })
  .await;
}
