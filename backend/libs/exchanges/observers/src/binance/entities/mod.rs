mod book_ticker;
mod error;
mod result;
mod subscribe;

use ::serde::{Deserialize, Serialize};

pub use self::subscribe::{SubscribeRequest, SubscribeRequestInner};

pub use self::book_ticker::BookTicker;
pub use self::error::Error as ErrorValue;
pub use self::result::ResultValue;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum WebsocketPayload {
  Error(ErrorValue),
  BookTicker(BookTicker<String>),
  Result(ResultValue),
}
