use ::async_trait::async_trait;
use ::futures::future::{join_all, select_all};
use ::futures::StreamExt;
use ::nats::asynk::Connection as Broker;
use ::rmp_serde::{from_slice as from_msgpack, to_vec as to_msgpack};
use ::slog::Logger;
use ::std::time::Duration;
use ::tokio::select;
use ::tokio::time::interval;
use ::tokio_tungstenite::connect_async;
use ::tokio_tungstenite::tungstenite::client::IntoClientRequest;
use ::tokio_tungstenite::tungstenite::Message;

use ::types::GenericResult;

use super::client::PubClient;
use super::constants::REST_ENDPOINT;
use super::constants::{USER_STREAM_LISTEN_KEY_SUB_NAME, WS_ENDPOINT};
use super::entities::{ListenKey, ListenKeyPair};

use crate::entities::APIKey;
use crate::errors::WebsocketError;
use crate::traits::UserStream as UserStreamTrait;
use crate::types::TLSWebSocket;

#[derive(Debug, Clone)]
pub struct UserStream {
  broker: Broker,
  logger: Logger,
}

impl UserStream {
  pub fn new(broker: Broker, logger: Logger) -> Self {
    return Self { broker, logger };
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
    let status = &resp.status();
    if !status.is_informational() {
      return Err(WebsocketError {
        status: Some(status.as_u16()),
        msg: status.canonical_reason().map(|s| s.to_string()),
      });
    }
    return Ok(socket);
  }
  async fn handle_message(&self, msg: &Message) -> GenericResult<()> {
    return Ok(());
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
    let key = ListenKeyPair::new(resp.listen_key, api_key.pub_key.clone());
    let _ = self
      .broker
      .publish(USER_STREAM_LISTEN_KEY_SUB_NAME, to_msgpack(&key)?)
      .await?;
    return Ok(());
  }
  async fn start(&self) -> GenericResult<()> {
    let mut listen_key_sub = self
      .broker
      .queue_subscribe(USER_STREAM_LISTEN_KEY_SUB_NAME, "user_stream")
      .await?
      .map(|msg| from_msgpack::<ListenKeyPair>(&msg.data))
      .filter_map(|msg| async { msg.ok() })
      .boxed();
    // 1800 = 30 * 60 = 30 mins.
    let mut listen_key_refresh = interval(Duration::from_secs(1800));
    let mut listen_keys: Vec<ListenKeyPair> = vec![];
    let mut user_stream: Vec<TLSWebSocket> = vec![];
    let me = self;
    loop {
      select! {
        Some(listen_key) = listen_key_sub.next() => {
          let socket = match me.init_websocket(
            format!("{}/{}", WS_ENDPOINT, listen_key.listen_key)
          ).await {
            Err(e) => {
              ::slog::warn!(
                me.logger, "Switching Protocol Failed"; e
              );
              continue;
            },
            Ok(v) => v,
          };
          user_stream.push(socket);
          listen_keys.push(listen_key);
        },
        _ = listen_key_refresh.tick() => {
          let url = format!("{}/api/v3/userDataStream", REST_ENDPOINT);
          let result_defer = listen_keys
            .iter().map(|key_pair| {
              return me
                .get_client(&key_pair.pub_key)
                .map(|cli| (cli, key_pair));
            })
            .filter_map(|res| res.ok())
            .map(|(cli, key_pair)| {
              return cli
                .put(url.as_str())
                .query(&[("listenKey", key_pair.listen_key.to_owned())])
                .send();
            });
          join_all(result_defer).await;
        },
        (Some(user_data), _, _) = select_all(
          user_stream.iter_mut().map(|stream| stream.next())
        ) => {
          let user_data = match user_data {
            Err(e) => {
              ::slog::warn!(me.logger, "Failed to receive payload: {}", e);
              continue;
            },
            Ok(v) => v,
          };
          me.handle_message(&user_data).await?;
        },
      };
    }
    return Ok(());
  }
}
