use ::std::time::Duration;

use ::rand::{thread_rng, Rng};
use ::uuid::{Builder, Bytes};

use ::errors::{DLockError, DLockResult};
use ::kvs::redis::Commands;
use ::kvs::{kvs, Store, WriteOption};

kvs!(pub, InitLock, String, "init_lock.{}");
kvs!(pub, InitFinLock, String, "init_fin_lock.{}");

pub fn lock<F, S, T>(mut dlock: T, func_on_success: F) -> DLockResult<()>
where
  F: Fn(),
  T: Store<S, String>,
  S: Commands,
{
  let mut rng = thread_rng();
  let seed: Bytes = rng.gen();
  let random = Builder::from_random_bytes(seed);
  let random = random.as_uuid();
  let lock: String = dlock.set(
    "init_lock",
    random.to_string(),
    WriteOption::default()
      .duration(Duration::from_secs(3).into())
      .non_existent_only(true)
      .into(),
  )?;
  if lock == "OK" {
    func_on_success();
    dlock.del("init_lock")?;
    return Ok(());
  }
  return Err(DLockError::CastFailure("Failed to acquire lock"));
}
