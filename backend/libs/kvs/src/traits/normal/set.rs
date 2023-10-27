use ::std::sync::Arc;

use ::async_trait::async_trait;

use ::errors::KVSResult;

use crate::options::WriteOption;
use crate::traits::base::Set as Base;

#[async_trait]
pub trait Set: Base {
  async fn set(
    &self,
    key: Arc<String>,
    value: Self::Value,
    opt: Option<WriteOption<Self::Commands>>,
  ) -> KVSResult<bool> {
    return self.__set__(key, value, Arc::new(opt)).await;
  }
}
