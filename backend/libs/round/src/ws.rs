use ::log::error;
use ::rand::thread_rng;
use ::rand::Rng;
use ::std::time::Duration;
use ::tokio::time::interval;
use ::tokio_tungstenite::connect_async;

use ::errors::{MaximumAttemptExceeded, WebSocketInitResult};
use ::types::TLSWebSocket;

pub struct WebSocket {
  socket: TLSWebSocket,
  endpoints: Vec<String>,
}

impl WebSocket {
  pub async fn connect(
    endpoints: &[&str],
  ) -> WebSocketInitResult<TLSWebSocket> {
    let mut interval = interval(Duration::from_secs(1));
    let url_index = {
      let mut rng = thread_rng();
      rng.gen_range(0..endpoints.len())
    };
    for jitter in 0..endpoints.len() {
      let url = endpoints[(url_index + jitter) % endpoints.len()].to_string();
      let (socket, resp) = connect_async(&url).await?;
      if resp.status().is_client_error() || resp.status().is_server_error() {
        let message = resp.body();
        let message = message
          .as_ref()
          .map(|b| String::from_utf8_lossy(b.as_slice()));
        error!(
          code = resp.status().as_u16(),
          message = message;
          "Failed to establish websocket connection:"
        );
      } else {
        return Ok(socket);
      }
      interval.tick().await;
    }
    return Err(MaximumAttemptExceeded.into());
  }

  pub async fn new(endpoints: &[&str]) -> WebSocketInitResult<Self> {
    let socket = Self::connect(endpoints).await?;
    return Ok(Self {
      socket,
      endpoints: endpoints.into_iter().map(|s| s.to_string()).collect(),
    });
  }
}
