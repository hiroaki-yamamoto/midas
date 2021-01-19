use ::async_trait::async_trait;
use ::futures::StreamExt;
use ::nats::asynk::Connection as Broker;
use ::tokio::select;
use ::tokio_tungstenite::connect_async;

use ::types::GenericResult;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

use super::client::PubClient;
use super::constants::REST_ENDPOINT;
use super::constants::{USER_STREAM_LISTEN_KEY_SUB_NAME, WS_ENDPOINT};
use super::entities::ListenKey;

use crate::entities::APIKey;
use crate::errors::WebsocketError;
use crate::traits::UserStream as UserStreamTrait;
use crate::types::TLSWebSocket;

#[derive(Debug, Clone)]
pub struct UserStream {
  broker: Broker,
}

impl UserStream {
  pub fn new(broker: Broker) -> Self {
    return Self { broker };
  }
  async fn init_websocket<S>(
    &self,
    addr: S,
  ) -> Result<TLSWebSocket, WebsocketError>
  where
    S: IntoClientRequest + Unpin,
  {
    let (socket, resp) =
      connect_async(addr).await.map_err(|err| WebsocketError {
        status: None,
        msg: Some(err.to_string()),
      })?;
    return Ok(socket);
  }
}

impl PubClient for UserStream {}

#[async_trait]
impl UserStreamTrait for UserStream {
  async fn authenticate(&mut self, api_key: &APIKey) -> GenericResult<()> {
    let client = self.get_client(api_key.pub_key.to_owned())?;
    let resp: ListenKey = client
      .post(format!("{}/api/v3/userDataStream", REST_ENDPOINT).as_str())
      .send()
      .await?
      .json()
      .await?;
    let _ = self
      .broker
      .publish(USER_STREAM_LISTEN_KEY_SUB_NAME, resp.listen_key.as_bytes())
      .await?;
    return Ok(());
  }
  async fn start(&self) -> GenericResult<()> {
    let listen_keys: Vec<String> = vec![];
    let mut listen_key_sub = self
      .broker
      .queue_subscribe(USER_STREAM_LISTEN_KEY_SUB_NAME, "user_stream")
      .await?
      .map(|msg| String::from_utf8(msg.data))
      .filter_map(|msg| async { msg.ok() })
      .boxed();
    let user_stream: Vec<TLSWebSocket> = vec![];
    loop {
      select! {
        Some(listen_key) = listen_key_sub.next() => {
          let (socket, resp) = connect_async(
            format!("{}/{}", WS_ENDPOINT, listen_key)
          ).await?;
        },
      };
    }
    return Ok(());
  }
}
