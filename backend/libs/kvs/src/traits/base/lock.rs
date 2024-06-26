use ::std::pin::Pin;
use ::std::sync::Arc;

use ::std::time::Duration;

use ::async_trait::async_trait;
use ::futures::future::{join, BoxFuture};
use ::random::generate_random_txt;
use ::tokio::select;
use ::tokio::sync::mpsc::channel;
use ::tokio::time::interval;

use ::errors::{DLockError, DLockResult};
use ::redis::{AsyncCommands as Commands, RedisError};

use super::{Base, ChannelName};
use crate::options::WriteOption;

#[async_trait]
pub trait Lock: Base + ChannelName {
  type Value: Send;

  async fn __lock__(
    &self,
    key: Arc<String>,
    func_on_success: Pin<
      Box<dyn (Fn() -> BoxFuture<'async_trait, Self::Value>) + Send + Sync>,
    >,
  ) -> DLockResult<Self::Value> {
    let (refresh_tx, mut refresh_rx) = channel::<()>(1);
    let channel_name = self.__channel_name__(format!("{}:lock", key).into());
    let channel_name_2 = channel_name.clone();
    let mut dlock = self.__commands__();
    let mut dlock2 = self.__commands__();

    let expire_refresh = async move {
      let _ = refresh_rx.recv().await;
      let mut refresh_timer = interval(Duration::from_secs(1));
      loop {
        select! {
          _ = refresh_timer.tick() => {
            // let mut dlock = dlock2.lock().await;
            let _ = dlock2.expire::<_, i64>(channel_name_2.as_ref(), 3).await;
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
        dlock
          .set_options(
            channel_name.as_ref(),
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
        let _ = async { dlock.del::<_, usize>("lock").await }.await;
        Ok::<Self::Value, DLockError>(res)
      } else {
        Err(DLockError::CastFailure("Failed to acquire lock"))
      }
    };
    return join(expire_refresh, acquire_process).await.1;
  }
}
