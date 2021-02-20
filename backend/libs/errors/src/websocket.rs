use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FormatResult};

use ::serde::Serialize;
use ::slog_derive::KV;

#[derive(Debug, Clone, Serialize, Default, KV)]
pub struct WebsocketError {
  pub status: Option<u16>,
  pub msg: Option<String>,
}

impl Display for WebsocketError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Websocket Error");
  }
}

impl Error for WebsocketError {}
