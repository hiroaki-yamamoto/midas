use ::std::collections::HashMap;
use ::std::time::Duration;
use core::future;

use ::async_trait::async_trait;
use ::futures::future::{join, join_all, select_all, FutureExt};
use ::futures::Sink;
use ::nats::asynk::Connection as Broker;
use ::rmp_serde::{from_slice as from_msgpack, to_vec as to_msgpack};
use ::slog::Logger;
use ::tokio::select;
use ::tokio::time::{interval, sleep};
use ::tokio_stream::{StreamExt, StreamMap};
use ::tokio_tungstenite::connect_async;
use ::tokio_tungstenite::tungstenite::{
  client::IntoClientRequest, Error as WebSocketError, Message,
};
use futures::SinkExt;

use ::types::GenericResult;

use super::client::PubClient;
use super::constants::REST_ENDPOINT;
use super::constants::{
  USER_STREAM_LISTEN_KEY_SUB_NAME, USER_STREAM_REAUTH_SUB_NAME, WS_ENDPOINT,
};
use super::entities::{ListenKey, ListenKeyPair};

use crate::entities::APIKey;
use crate::errors::{MaximumAttemptExceeded, WebsocketError};
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
  async fn handle_message(
    &self,
    api_key: &String,
    sockets: &mut StreamMap<String, TLSWebSocket>,
    listen_keys: &mut HashMap<String, String>,
    msg: &Message,
  ) -> GenericResult<()> {
    if let Message::Close(reason) = &msg {
      match reason {
        Some(reason) => {
          ::slog::warn!(
            self.logger, "Closing connection...";
            "code" => format!("{}", reason.code),
            "reason" => reason.reason.to_string()
          );
        }
        None => {
          ::slog::warn!(self.logger, "Closing connection...");
        }
      };
      if let Some(mut socket) = sockets.remove(api_key) {
        let _ = socket.close(None).await;
      }
      listen_keys.remove(api_key);
      let _ = self
        .broker
        .publish(USER_STREAM_REAUTH_SUB_NAME, api_key)
        .await;
      return Ok(());
    }
    let socket_opt = sockets
      .iter_mut()
      .find(|(pub_key, _)| pub_key == api_key)
      .map(|(_, socket)| socket);
    if let Some(socket) = socket_opt {
      match msg {
        Message::Ping(d) => {
          let _ = socket.send(Message::Pong(d.to_owned())).await;
        }
        Message::Binary(binary) => {}
        Message::Text(text) => {}
        _ => {}
      };
      let _ = socket.flush().await;
    }
    return Ok(());
  }

  async fn handle_disconnect(
    &self,
    pub_key: &String,
  ) -> Result<(), MaximumAttemptExceeded> {
    let retry_sec = Duration::from_secs(5);
    ::slog::warn!(
      self.logger,
      "Session Disconnected. Reconnecting...";
      "api_key" => &pub_key,
    );
    let mut key = APIKey::default();
    key.pub_key = pub_key.clone();
    for _ in 0..5 {
      match self.authenticate(&key).await {
        Err(e) => {
          ::slog::warn!(
            self.logger,
            "Failed to reconnect trying in {} secs ({})",
            retry_sec.as_secs(),
            e; "pub_key" => &key.pub_key
          );
        }
        Ok(_) => {
          ::slog::info!(
            self.logger,
            "Reconnected.";
            "pub_key" => &key.pub_key
          );
          return Ok(());
        }
      }
      sleep(retry_sec).await;
    }
    return Err(MaximumAttemptExceeded);
  }
}

impl PubClient for UserStream {}

#[async_trait]
impl UserStreamTrait for UserStream {
  async fn authenticate(&self, api_key: &APIKey) -> GenericResult<()> {
    let client = self.get_client(api_key.pub_key.to_owned())?;
    let resp: ListenKey = client
      .post(format!("{}/api/v3/userDataStream", REST_ENDPOINT).as_str())
      .send()
      .await?
      .error_for_status()?
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
    let (listen_key_sub, reauth_sub) = join(
      self
        .broker
        .queue_subscribe(USER_STREAM_LISTEN_KEY_SUB_NAME, "user_stream"),
      self
        .broker
        .queue_subscribe(USER_STREAM_REAUTH_SUB_NAME, "user_stream"),
    )
    .await;
    let listen_key_sub = listen_key_sub?
      .map(|msg| from_msgpack::<ListenKeyPair>(&msg.data))
      .filter_map(|msg| msg.ok());
    let reauth_sub = reauth_sub?
      .map(|msg| String::from_utf8(msg.data))
      .filter_map(|msg| msg.ok());
    let mut listen_key_sub = Box::pin(listen_key_sub);
    let mut reauth_sub = Box::pin(reauth_sub);
    // 1800 = 30 * 60 = 30 mins.
    let mut listen_key_refresh = interval(Duration::from_secs(1800));
    let mut sockets: StreamMap<String, TLSWebSocket> = StreamMap::new();
    // Key = Pub API key, Value = Listen Key
    let mut listen_keys: HashMap<String, String> = HashMap::new();
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
          sockets.insert(listen_key.pub_key.clone(), socket);
          listen_keys.insert(listen_key.pub_key, listen_key.listen_key);
        },
        _ = listen_key_refresh.tick() => {
          let url = format!("{}/api/v3/userDataStream", REST_ENDPOINT);
          let result_defer = listen_keys
            .iter().map(|(pub_key, lis_key)| {
              return me
                .get_client(&pub_key)
                .map(|cli| (cli, lis_key));
            })
            .filter_map(|res| res.ok())
            .map(|(cli, lis_key)| {
              return cli
                .put(url.as_str())
                .query(&[("listenKey", lis_key.to_owned())])
                .send();
            });
          join_all(result_defer).await;
        },
        Some(pub_key) = reauth_sub.next() => {
          match me.handle_disconnect(&pub_key).await {
            Ok(_) => {},
            Err(e) => {
              ::slog::error!(
                me.logger,
                "Failed to authenticate listen key: {}", e
              );
            },
          };
        },
        Some((api_key, msg)) = sockets.next() => {
          // I have no idea to handle the dirty close...
          let user_data = match msg {
            Err(e) => {
              match e {
                WebSocketError::ConnectionClosed |
                WebSocketError::AlreadyClosed => {
                  sockets.remove(&api_key);
                  listen_keys.remove(&api_key);
                  let _ = me.broker.publish(
                    USER_STREAM_REAUTH_SUB_NAME,
                    api_key
                  ).await;
                },
                _ => ::slog::warn!(me.logger, "Failed to receive payload: {}", e),
              }
              continue;
            },
            Ok(v) => v,
          };
          me.handle_message(&api_key, &mut sockets, &mut listen_keys, &user_data).await?;
        }
      };
    }
    return Ok(());
  }
}
