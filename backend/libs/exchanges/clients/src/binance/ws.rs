use ::rand::thread_rng;
use ::rand::Rng;
use ::tokio_tungstenite::connect_async;

use ::errors::{StatusFailure, WebSocketInitResult};
use ::types::TLSWebSocket;

use super::WS_ENDPOINT;

pub struct WebSocket {
  socket: TLSWebSocket,
}

impl WebSocket {
  fn choose_url() -> String {
    let mut rng = thread_rng();
    let index: usize = rng.gen_range(0..WS_ENDPOINT.len());
    return WS_ENDPOINT[index].to_string();
  }
  pub async fn new() -> WebSocketInitResult<Self> {
    let url = Self::choose_url();
    let (socket, resp) = connect_async(&url).await?;
    if resp.status().is_client_error() || resp.status().is_server_error() {
      let err = StatusFailure::new(
        Some(url),
        resp.status().as_u16(),
        "Failed to establish websocket connection".to_string(),
      );
      return Err(err.into());
    }
    return Ok(Self { socket });
  }
}
