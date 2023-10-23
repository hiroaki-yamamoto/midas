use ::std::future::Future;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::futures::future::join;
use ::random::generate_random_txt;
use ::tokio::select;
use ::tokio::sync::mpsc::channel;
use ::tokio::time::interval;

use ::errors::{DLockError, DLockResult};
use ::redis::{AsyncCommands as Commands, RedisError};

use super::{Base, ChannelName};
use crate::options::WriteOption;

#[async_trait]
pub trait Lock<S, Ft, Fr>: Base<S> + ChannelName
where
  S: Commands + Send,
{
  async fn lock(
    &self,
    key: &str,
    func_on_success: impl (Fn() -> Ft) + Send + Sync,
  ) -> DLockResult<Fr> {
    let (refresh_tx, mut refresh_rx) = channel::<()>(1);
    let channel_name = self.channel_name(&format!("{}:lock", key));
    let channel_name_2 = channel_name.clone();
    let mut dlock = self.commands();
    let mut dlock2 = self.commands();

    let expire_refresh = async move {
      let _ = refresh_rx.recv().await;
      let mut refresh_timer = interval(Duration::from_secs(1));
      loop {
        select! {
          _ = refresh_timer.tick() => {
            // let mut dlock = dlock2.lock().await;
            let _ = dlock2.expire::<_, i64>(channel_name_2.clone(), 3).await;
          },
          _ = refresh_rx.recv() => {
            break;
          },
        }
      }
      Ok::<(), RedisError>(())
    };
    let acquire_process = async move {
      let random = generate_random_txt(32);
      let lock: String = async {
        // let mut dlock = dlock.lock().await;
        dlock
          .set_options(
            channel_name,
            random.to_string(),
            WriteOption::default()
              .duration(Duration::from_secs(3).into())
              .non_existent_only(true)
              .into(),
          )
          .await
      }
      .await?;
      if lock == "OK" {
        let _ = refresh_tx.send(());
        let res = func_on_success().await;
        let _ = refresh_tx.send(());
        let _ = async {
          // let mut dlock = dlock.lock().await;
          dlock.del::<_, usize>("lock").await
        }
        .await;
        Ok::<Fr, DLockError>(res)
      } else {
        Err(DLockError::CastFailure("Failed to acquire lock"))
      }
    };
    return join(expire_refresh, acquire_process).await.1;
  }
}
