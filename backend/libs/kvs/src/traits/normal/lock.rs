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
    func_on_success: Arc<
      dyn (Fn() -> BoxFuture<'async_trait, Arc<dyn Send + Sync>>) + Send + Sync,
    >,
  ) -> DLockResult<Arc<dyn Send + Sync>> {
    return self.__lock__(key, func_on_success).await;
  }
}
