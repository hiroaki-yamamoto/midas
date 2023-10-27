use ::std::future::Future;
use ::std::sync::Arc;

use ::async_trait::async_trait;

use ::errors::DLockResult;

use crate::traits::base::Lock as Base;

#[async_trait]
pub trait Lock: Base {
  async fn lock(
    &self,
    key: Arc<String>,
    func_on_success: impl (Fn() -> Arc<dyn Future<Output = Arc<dyn Send>>>)
      + Send
      + Sync,
  ) -> DLockResult<Arc<dyn Send>> {
    return self.__lock__(key, func_on_success).await;
  }
}
