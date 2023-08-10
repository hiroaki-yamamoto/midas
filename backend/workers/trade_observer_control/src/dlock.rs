use ::std::sync::{Arc, Mutex};
use ::std::time::Duration;

use ::futures::future::join;
use ::rand::{thread_rng, Rng};
use ::tokio::select;
use ::tokio::sync::mpsc::channel;
use ::tokio::time::interval;
use ::uuid::{Builder, Bytes};

use ::errors::{DLockError, DLockResult};
use ::kvs::redis::{Commands, RedisError};
use ::kvs::{kvs, Store, WriteOption};

kvs!(pub, InitLock, String, "init_lock:{}");
kvs!(pub, InitFinLock, String, "init_fin_lock:{}");

pub async fn lock<F, S, T>(dlock: T, func_on_success: F) -> DLockResult<()>
where
  F: Fn(),
  T: Store<S, String>,
  S: Commands,
{
  let (refresh_tx, mut refresh_rx) = channel::<()>(1);
  let dlock = Arc::new(Mutex::new(dlock));
  let dlock2 = dlock.clone();
  let expire_refresh = async move {
    let _ = refresh_rx.recv().await;
    let mut refresh_timer = interval(Duration::from_secs(1));
    let mut dlock = dlock2.lock().unwrap();
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
    let mut rng = thread_rng();
    let seed: Bytes = rng.gen();
    let random = Builder::from_random_bytes(seed);
    let random = random.as_uuid();
    let mut dlock = dlock.lock().unwrap();
    let lock: String = dlock.set(
      "lock",
      random.to_string(),
      WriteOption::default()
        .duration(Duration::from_secs(3).into())
        .non_existent_only(true)
        .into(),
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
