use ::async_trait::async_trait;

use ::errors::WebsocketMessageResult;

use crate::entities::WSMessageDetail as MsgDetail;

#[async_trait]
pub trait IWebSocketStream {
  type Item: Send;
  async fn next(&mut self) -> WebsocketMessageResult<MsgDetail<Self::Item>>;
}
