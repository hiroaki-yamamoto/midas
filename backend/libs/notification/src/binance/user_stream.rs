use ::std::collections::HashMap;
use ::std::sync::Arc;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::futures::future::try_join_all;
use ::futures::{SinkExt, StreamExt};
use ::log::{as_display, as_error, error, info, warn};
use ::serde_json::{from_slice as from_json_bin, from_str as from_json_str};
use ::tokio::time::{interval, sleep};
use ::tokio::{join, select};
use ::tokio_stream::StreamMap;
use ::tokio_tungstenite::connect_async;
use ::tokio_tungstenite::tungstenite::{
  client::IntoClientRequest, Error as WebSocketError, Message,
};

use ::clients::binance::WS_ENDPOINT;
use ::errors::{MaximumAttemptExceeded, NotificationResult, WebsocketError};
use ::keychain::binance::APIKeySigner;
use ::keychain::pubsub::APIKeyPubSub;
use ::keychain::{APIKey, APIKeyEvent};
use ::subscribe::nats::Client as Nats;
use ::subscribe::PubSub;
use ::types::TLSWebSocket;

use crate::traits::UserStream as UserStreamTrait;

use super::entities::{
  CastedUserStreamEvents, ListenKey, ListenKeyPair, RawUserStreamEvents,
};
use super::interfaces::IListenKeyClient;
use super::pubsub::{ListenKeyPubSub, NotifyPubSub, ReauthPubSub};
use super::services::ListenKeyClient;

pub struct UserStream {
  key_pubsub: APIKeyPubSub,
  notify_pubsub: NotifyPubSub,
  reauth_pubsub: ReauthPubSub,
  listen_key_pubsub: ListenKeyPubSub,
  cli: Arc<dyn IListenKeyClient + Send + Sync>,
}

impl UserStream {
  pub async fn new(broker: &Nats) -> NotificationResult<Self> {
    let (key_pubsub, notify_pubsub, reauth_pubsub, listen_key_pubsub) = join!(
      APIKeyPubSub::new(broker),
      NotifyPubSub::new(broker),
      ReauthPubSub::new(broker),
      ListenKeyPubSub::new(broker),
    );
    let (key_pubsub, notify_pubsub, reauth_pubsub, listen_key_pubsub) = (
      key_pubsub?,
      notify_pubsub?,
      reauth_pubsub?,
      listen_key_pubsub?,
    );
    let signer = Arc::new(APIKeySigner::new());
    let cli = Arc::new(ListenKeyClient::new(signer)?);
    return Ok(Self {
      key_pubsub,
      notify_pubsub,
      reauth_pubsub,
      listen_key_pubsub,
      cli,
    });
  }
  async fn init_websocket<S>(&self, addr: S) -> NotificationResult<TLSWebSocket>
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
      return Err(
        WebsocketError {
          status: Some(status.as_u16()),
          msg: status.canonical_reason().map(|s| s.to_string()),
        }
        .into(),
      );
    }
    return Ok(socket);
  }
  async fn handle_user_stream_event(
    &self,
    uds: RawUserStreamEvents,
  ) -> NotificationResult<()> {
    let _ = self
      .notify_pubsub
      .publish({
        let casted: NotificationResult<CastedUserStreamEvents> = uds.into();
        &casted?
      })
      .await?;
    return Ok(());
  }
  async fn handle_message(
    &self,
    api_key: Arc<APIKey>,
    sockets: &mut StreamMap<Arc<APIKey>, TLSWebSocket>,
    listen_keys: &mut HashMap<Arc<APIKey>, Arc<ListenKey>>,
    msg: &Message,
  ) -> NotificationResult<()> {
    if let Message::Close(reason) = &msg {
      match reason {
        Some(reason) => {
          warn!(
            code = as_display!(reason.code),
            reason = reason.reason;
            "Closing connection...",
          );
        }
        None => {
          warn!("Closing connection...");
        }
      };
      if let Some(mut socket) = sockets.remove(&api_key) {
        let _ = socket.close(None).await;
      }
      listen_keys.remove(&api_key);
      let _ = self.reauth_pubsub.publish(api_key.as_ref()).await;
      return Ok(());
    }
    let socket_opt = sockets
      .iter_mut()
      .find(|(pub_key, _)| *pub_key == api_key)
      .map(|(_, socket)| socket);
    if let Some(socket) = socket_opt {
      match msg {
        Message::Ping(d) => {
          let _ = socket.send(Message::Pong(d.to_owned())).await;
        }
        Message::Binary(binary) => {
          let event: RawUserStreamEvents = from_json_bin(binary)?;
          self.handle_user_stream_event(event).await?;
        }
        Message::Text(text) => {
          let event: RawUserStreamEvents = from_json_str(text.as_str())?;
          self.handle_user_stream_event(event).await?;
        }
        _ => {}
      };
      let _ = socket.flush().await;
    }
    return Ok(());
  }

  async fn handle_disconnect(
    &self,
    api_key: Arc<APIKey>,
  ) -> NotificationResult<()> {
    let retry_sec = Duration::from_secs(5);
    let inner = api_key.as_ref().inner();
    warn!(key = inner.pub_key; "Session Disconnected. Reconnecting...",);
    for _ in 0..5 {
      match self.get_listen_key(api_key.clone()).await {
        Err(e) => {
          warn!(
            pub_key = inner.pub_key,
            error = as_display!(e);
            "Failed to reconnect trying in {} secs",
            retry_sec.as_secs(),
          );
        }
        Ok(_) => {
          info!("pub_key" = inner.pub_key; "Reconnected.");
          return Ok(());
        }
      }
      sleep(retry_sec).await;
    }
    return Err(MaximumAttemptExceeded.into());
  }

  async fn get_listen_key(
    &self,
    api_key: Arc<APIKey>,
  ) -> NotificationResult<()> {
    let listen_key = self.cli.create(api_key.clone()).await?;
    let key = ListenKeyPair::new(listen_key.listen_key, api_key);
    let _ = self.listen_key_pubsub.publish(&key).await?;
    return Ok(());
  }

  async fn close_listen_key(
    &self,
    api_key: Arc<APIKey>,
    listen_key: Arc<ListenKey>,
  ) -> NotificationResult<()> {
    let _ = self.cli.delete(api_key, listen_key).await?;
    return Ok(());
  }
}

#[async_trait]
impl UserStreamTrait for UserStream {
  async fn start(&self) -> NotificationResult<()> {
    let (keychain_sub, listen_key_sub, reauth_sub) = join!(
      self.key_pubsub.pull_subscribe("userStream"),
      self.listen_key_pubsub.pull_subscribe("userStream"),
      self.reauth_pubsub.pull_subscribe("userStream")
    );
    let (keychain_sub, listen_key_sub, reauth_sub) =
      (keychain_sub?, listen_key_sub?, reauth_sub?);
    let mut keychain_sub = keychain_sub.boxed();
    let mut listen_key_sub = listen_key_sub.boxed();
    let mut reauth_sub = reauth_sub.boxed();
    // 1800 = 30 * 60 = 30 mins.
    let mut listen_key_refresh = interval(Duration::from_secs(1800));
    let mut sockets: StreamMap<Arc<APIKey>, TLSWebSocket> = StreamMap::new();
    // Key = Pub API key, Value = Listen Key
    let mut listen_keys: HashMap<Arc<APIKey>, Arc<ListenKey>> = HashMap::new();
    loop {
      select! {
        Some((event, _)) = keychain_sub.next() => {
          match event {
            APIKeyEvent::Add(api_key) => {
              let _ = self.get_listen_key(Arc::new(api_key)).await;
            },
            APIKeyEvent::Remove(api_key) => {
              if let Some(listen_key) = listen_keys.remove(&api_key) {
                sockets.remove(&api_key);
                let _ = self.close_listen_key(Arc::new(api_key), listen_key);
              }
            },
          }
        },
        Some((listen_key, _)) = listen_key_sub.next() => {
          let socket = match self.init_websocket(
            format!("{}/{}", WS_ENDPOINT[0], listen_key.listen_key)
          ).await {
            Err(e) => {
              warn!(error = as_display!(e); "Switching Protocol Failed");
              continue;
            },
            Ok(v) => v,
          };
          let api_key = listen_key.api_key.clone();
          sockets.insert(api_key.clone(), socket);
          listen_keys.insert(api_key.clone(), Arc::new((&listen_key).into()));
        },
        _ = listen_key_refresh.tick() => {
          let result_defer: Vec<_> = listen_keys
            .iter().map(|(api_key, lis_key)| {
              self.cli.extend_lifetime(api_key.clone(), lis_key.clone())
            }).collect();
          if let Err(e) = try_join_all(result_defer).await {
            error!(error = as_error!(e); "Failed to extend listen key lifetime");
          }
        },
        Some((api_key, _)) = reauth_sub.next() => {
          match self.handle_disconnect(Arc::new(api_key)).await {
            Ok(_) => {},
            Err(e) => {
              error!(
                error = as_display!(e); "Failed to authenticate listen key",
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
                  let _ = self.reauth_pubsub.publish(&api_key).await;
                },
                _ => warn!(error = as_display!(e); "Failed to receive payload"),
              }
              continue;
            },
            Ok(v) => v,
          };
          self.handle_message(api_key, &mut sockets, &mut listen_keys, &user_data).await?;
        }
      };
    }
  }
}
