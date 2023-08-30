use ::async_trait::async_trait;
use ::futures::stream::BoxStream;
use ::mongodb::error::Result as DBResult;

#[async_trait]
pub trait ListTradingSymbols {
  async fn list_trading(&self) -> DBResult<BoxStream<String>>;
}
