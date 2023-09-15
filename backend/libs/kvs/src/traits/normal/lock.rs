use ::std::future::Future;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::futures::future::join;
use ::rand::distributions::Alphanumeric;
use ::rand::{thread_rng, Rng};
use ::tokio::select;
use ::tokio::sync::mpsc::channel;
use ::tokio::time::interval;

use ::errors::{DLockError, DLockResult};
use ::redis::{Commands, RedisError};

use super::{Base, ChannelName};
use crate::options::WriteOption;

fn rand_txt(len: usize) -> String {
  let rand_string: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(len)
    .map(char::from)
    .collect();
  return rand_string;
}

#[async_trait]
pub trait Lock<S>: Base<S> + ChannelName
where
  S: Commands + Send,
{
  async fn lock<Ft>(
    &mut self,
    key: &str,
    func_on_success: impl (Fn() -> Ft) + Send + Sync,
  ) -> DLockResult<()>
  where
    Ft: Future<Output = ()> + Send,
  {
    let (refresh_tx, mut refresh_rx) = channel::<()>(1);
    let channel_name = self.channel_name(&format!("{}:lock", key));
    let channel_name_2 = channel_name.clone();
    let dlock = self.commands();
    let dlock2 = self.commands();

    let expire_refresh = async move {
      let _ = refresh_rx.recv().await;
      let mut refresh_timer = interval(Duration::from_secs(1));
      let mut dlock = dlock2.lock().await;
      loop {
        select! {
          _ = refresh_timer.tick() => {
            let _ = dlock.expire::<_, i64>(channel_name_2.clone(), 3);
          },
          _ = refresh_rx.recv() => {
            break;
          },
        }
      }
      Ok::<(), RedisError>(())
    };
    let acquire_process = async move {
      let random = rand_txt(32);
      let mut dlock = dlock.lock().await;
      let lock: String = dlock.set_options(
        channel_name,
        random.to_string(),
        WriteOption::default()
          .duration(Duration::from_secs(3).into())
          .non_existent_only(true)
          .into(),
      )?;
      if lock == "OK" {
        let _ = refresh_tx.send(());
        let _ = func_on_success().await;
        let _ = refresh_tx.send(());
        let _ = dlock.del::<_, usize>("lock");
        Ok::<(), DLockError>(())
      } else {
        Err(DLockError::CastFailure("Failed to acquire lock"))
      }
    };
    return join(expire_refresh, acquire_process).await.1;
  }
}
