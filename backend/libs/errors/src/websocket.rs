use ::err_derive::Error;

use ::serde::Serialize;
use ::slog_derive::KV;

#[derive(Debug, Clone, Serialize, Default, KV, Error)]
#[error(display = "Websocket Error (status: {:?}, msg: {:?})", status, msg)]
pub struct WebsocketError {
  pub status: Option<u16>,
  pub msg: Option<String>,
}
