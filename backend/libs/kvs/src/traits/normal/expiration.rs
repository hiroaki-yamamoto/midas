use ::std::sync::Arc;
use ::std::time::Duration;

use ::async_trait::async_trait;

use ::errors::KVSResult;

use crate::traits::base::Expiration as Base;

#[async_trait]
pub trait Expiration: Base {
  async fn expire(&self, key: Arc<String>, dur: Duration) -> KVSResult<bool> {
    return self.__expire__(key, dur).await;
  }
}
