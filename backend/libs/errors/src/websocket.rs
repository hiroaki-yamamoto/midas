use ::err_derive::Error;
use ::serde::Serialize;

#[derive(Debug, Clone, Serialize, Default, Error)]
#[error(display = "Websocket Error (status: {:?}, msg: {:?})", status, msg)]
pub struct WebsocketError {
  pub status: Option<u16>,
  pub msg: Option<String>,
}
