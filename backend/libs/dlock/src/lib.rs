use ::std::sync::Arc;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::futures::future::join;
use ::tokio::select;
use ::tokio::sync::mpsc::channel;
use ::tokio::sync::Mutex;
use ::tokio::time::interval;
use ::uuid::Uuid;

use ::errors::{DLockError, DLockResult};
use ::kvs::redis::{Commands, RedisError};
use ::kvs::{Store, WriteOption};

#[async_trait]
pub trait Dlock<S>: Store<S, String>
where
  S: Commands + Send,
{
  async fn lock(
    &mut self,
    func_on_success: impl Fn() + Send + Sync,
  ) -> DLockResult<()> {
    let (refresh_tx, mut refresh_rx) = channel::<()>(1);
    let dlock = Arc::new(Mutex::new(self));
    let dlock2 = dlock.clone();
    let expire_refresh = async move {
      let _ = refresh_rx.recv().await;
      let mut refresh_timer = interval(Duration::from_secs(1));
      let mut dlock = dlock2.lock().await;
      loop {
        select! {
          _ = refresh_timer.tick() => {
            let _ = dlock.expire("lock", Duration::from_secs(3));
          },
          _ = refresh_rx.recv() => {
            break;
          },
        }
      }
      Ok::<(), RedisError>(())
    };
    let acquire_process = async {
      let random = Uuid::new_v4();
      let mut dlock = dlock.lock().await;
      let lock: String = dlock.set(
        "lock",
        random.to_string(),
        Some(
          WriteOption::default()
            .duration(Duration::from_secs(3).into())
            .non_existent_only(true),
        ),
      )?;
      if lock == "OK" {
        let _ = refresh_tx.send(());
        func_on_success();
        let _ = refresh_tx.send(());
        dlock.del("lock")?;
        return Ok(());
      }
      Err(DLockError::CastFailure("Failed to acquire lock"))
    };
    return join(expire_refresh, acquire_process).await.1;
  }
}
