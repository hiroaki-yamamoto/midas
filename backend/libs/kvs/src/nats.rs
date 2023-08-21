use ::errors::{NatsKVSError, NatsKVSResult};
use ::rmp_serde::from_slice as msgpack_parse;
use serde::de::DeserializeOwned;

use ::subscribe::natsJS::kv::Store;

#[derive(Debug, Clone)]
pub struct NatsKVS {
  store: Store,
}

impl NatsKVS {
  pub fn new(store: Store) -> Self {
    return Self { store };
  }
  pub async fn get_last<S, T>(&self, key: S) -> NatsKVSResult<T>
  where
    S: AsRef<str>,
    T: DeserializeOwned,
  {
    if let Some(entry) = self.store.entry(key.as_ref()).await? {
      return Ok(msgpack_parse::<T>(&entry.value.to_vec())?);
    }
    return Err(NatsKVSError::NoValue);
  }
}
