mod book_ticker;
mod result;
mod subscribe;

use ::serde::Deserialize;

pub use self::subscribe::{SubscribeRequest, SubscribeRequestInner};

pub use self::book_ticker::BookTicker;
pub use self::result::ResultValue;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum WebsocketPayload {
  BookTicker(BookTicker<String>),
  Result(ResultValue),
}
