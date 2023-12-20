use ::async_trait::async_trait;

use ::errors::WebsocketMessageResult;

#[async_trait]
pub trait IWebSocketStream {
  type Item: Send;
  async fn next(&mut self) -> WebsocketMessageResult<Option<Self::Item>>;
}
