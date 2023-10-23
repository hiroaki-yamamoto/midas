use ::std::future::Future;

use ::async_trait::async_trait;

use ::errors::DLockResult;
use ::redis::AsyncCommands as Commands;

use crate::traits::base::Lock as Base;

#[async_trait]
pub trait Lock<S, Ft, Fr>: Base<S, Ft, Fr>
where
  S: Commands + Send,
  Ft: Future<Output = Fr> + Send,
  Fr: Send,
{
  async fn lock(
    &self,
    key: &str,
    func_on_success: impl (Fn() -> Ft) + Send + Sync,
  ) -> DLockResult<Fr> {
    return Base::lock(self, key, func_on_success).await;
  }
}
