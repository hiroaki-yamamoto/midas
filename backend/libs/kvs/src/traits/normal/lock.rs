use ::std::pin::Pin;
use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::futures::future::BoxFuture;

use ::errors::DLockResult;

use crate::traits::base::Lock as Base;

#[async_trait]
pub trait Lock: Base {
  async fn lock(
    &self,
    key: Arc<String>,
    func_on_success: Pin<
      Box<dyn (Fn() -> BoxFuture<'async_trait, Self::Value>) + Send + Sync>,
    >,
  ) -> DLockResult<Self::Value> {
    return self.__lock__(key, func_on_success).await;
  }
}
