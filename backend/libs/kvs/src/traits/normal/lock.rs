use ::std::future::Future;

use ::async_trait::async_trait;

use ::errors::DLockResult;
use ::redis::AsyncCommands as Commands;

use crate::traits::base::Lock as Base;

#[async_trait]
pub trait Lock<S>: Base<S>
where
  S: Commands + Send,
{
  async fn lock<Ft, Fr>(
    &self,
    key: &str,
    func_on_success: impl (Fn() -> Ft) + Send + Sync,
  ) -> DLockResult<Fr>
  where
    Ft: Future<Output = Fr> + Send,
    Fr: Send,
  {
    return Base::lock(self, key, func_on_success).await;
  }
}
