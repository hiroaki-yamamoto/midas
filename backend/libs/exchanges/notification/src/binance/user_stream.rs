use ::std::collections::HashMap;
use ::std::sync::Arc;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::futures::future::join_all;
use ::futures::{SinkExt, StreamExt};
use ::log::{as_display, error, info, warn};
use ::nats::jetstream::JetStream as NatsJS;
use ::serde_json::{from_slice as from_json_bin, from_str as from_json_str};
use ::tokio::sync::Mutex;
use ::tokio::time::{interval, sleep};
use ::tokio::{join, select};
use ::tokio_stream::StreamMap;
use ::tokio_tungstenite::connect_async;
use ::tokio_tungstenite::tungstenite::{
  client::IntoClientRequest, Error as WebSocketError, Message,
};

use ::clients::binance::{APIHeader, REST_ENDPOINTS, WS_ENDPOINT};
use ::round::RestClient;

use crate::traits::UserStream as UserStreamTrait;
use ::entities::{APIKey, APIKeyEvent, APIKeyInner};
use ::errors::{
  MaximumAttemptExceeded, NotificationResult, UserStreamResult, WebsocketError,
};
use ::keychain::pubsub::APIKeyPubSub;
use ::subscribe::PubSub;
use ::types::TLSWebSocket;

use super::entities::{
  CastedUserStreamEvents, ListenKey, ListenKeyPair, RawUserStreamEvents,
};
use super::pubsub::{ListenKeyPubSub, NotifyPubSub, ReauthPubSub};

#[derive(Debug, Clone)]
pub struct UserStream {
  key_pubsub: APIKeyPubSub,
  notify_pubsub: NotifyPubSub,
  reauth_pubsub: ReauthPubSub,
  listen_key_pubsub: ListenKeyPubSub,
  cli: RestClient,
}

impl UserStream {
  pub async fn new(broker: NatsJS) -> UserStreamResult<Self> {
    let (key_pubsub, notify_pubsub, reauth_pubsub, listen_key_pubsub) = join!(
      APIKeyPubSub::new(broker.clone()),
      NotifyPubSub::new(broker.clone()),
      ReauthPubSub::new(broker.clone()),
      ListenKeyPubSub::new(broker.clone()),
    );
    let (key_pubsub, notify_pubsub, reauth_pubsub, listen_key_pubsub) = (
      key_pubsub?,
      notify_pubsub?,
      reauth_pubsub?,
      listen_key_pubsub?,
    );
    return Ok(Self {
      key_pubsub,
      notify_pubsub,
      reauth_pubsub,
      listen_key_pubsub,
      cli: RestClient::new(
        REST_ENDPOINTS
          .into_iter()
          .filter_map(|endpoint| {
            format!("{}/api/v3/userDataStream", endpoint).parse().ok()
          })
          .collect(),
        Duration::from_secs(5),
        Duration::from_secs(5),
      )?,
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
    let _ = self.notify_pubsub.publish({
      let casted: NotificationResult<CastedUserStreamEvents> = uds.into();
      &casted?
    })?;
    return Ok(());
  }
  async fn handle_message(
    &self,
    api_key: &String,
    sockets: &mut StreamMap<String, TLSWebSocket>,
    listen_keys: &mut HashMap<String, String>,
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
      if let Some(mut socket) = sockets.remove(api_key) {
        let _ = socket.close(None).await;
      }
      listen_keys.remove(api_key);
      let _ = self.reauth_pubsub.publish(&api_key);
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
    &mut self,
    pub_key: &String,
  ) -> NotificationResult<()> {
    let retry_sec = Duration::from_secs(5);
    warn!(api_key = pub_key; "Session Disconnected. Reconnecting...",);
    let mut key = APIKeyInner::default();
    key.pub_key = pub_key.clone();
    for _ in 0..5 {
      match self.get_listen_key(&key).await {
        Err(e) => {
          warn!(
            pub_key = key.pub_key,
            error = as_display!(e);
            "Failed to reconnect trying in {} secs",
            retry_sec.as_secs(),
          );
        }
        Ok(_) => {
          info!("pub_key" = key.pub_key; "Reconnected.");
          return Ok(());
        }
      }
      sleep(retry_sec).await;
    }
    return Err(MaximumAttemptExceeded.into());
  }
}

impl APIHeader for UserStream {}

#[async_trait]
impl UserStreamTrait for UserStream {
  async fn get_listen_key(
    &mut self,
    api_key: &APIKeyInner,
  ) -> NotificationResult<()> {
    let header = self.get_pub_header(api_key)?;
    let resp: ListenKey = self
      .cli
      .post::<()>(Some(header), None)
      .await?
      .error_for_status()?
      .json()
      .await?;
    let key = ListenKeyPair::new(resp.listen_key, api_key.pub_key.clone());
    let _ = self.listen_key_pubsub.publish(&key)?;
    return Ok(());
  }
  async fn close_listen_key(
    &mut self,
    api_key: &APIKeyInner,
    listen_key: &String,
  ) -> NotificationResult<()> {
    let pub_key = self.get_pub_header(api_key)?;
    let _ = self
      .cli
      .delete(Some(pub_key), Some(&[("listenKey", listen_key)]))
      .await?
      .error_for_status()?;
    return Ok(());
  }
  async fn start(&mut self) -> NotificationResult<()> {
    let keychain_sub = self.key_pubsub.queue_subscribe("KeyChainUserStream")?;
    let listen_key_sub =
      self.listen_key_pubsub.queue_subscribe("KeyChainListener")?;
    let reauth_sub = self
      .reauth_pubsub
      .queue_subscribe("KeyChainReAuthListener")?;
    let mut keychain_sub = keychain_sub.boxed();
    let mut listen_key_sub = listen_key_sub.boxed();
    let mut reauth_sub = reauth_sub.boxed();
    // 1800 = 30 * 60 = 30 mins.
    let mut listen_key_refresh = interval(Duration::from_secs(1800));
    let mut sockets: StreamMap<String, TLSWebSocket> = StreamMap::new();
    // Key = Pub API key, Value = Listen Key
    let mut listen_keys: HashMap<String, String> = HashMap::new();
    let me = Arc::new(Mutex::new(self));
    loop {
      let me = Arc::clone(&me);
      select! {
        Some((event, _)) = keychain_sub.next() => {
          let mut me = me.lock().await;
          match event {
            APIKeyEvent::Add(APIKey::Binance(api_key)) => {
              let _ = me.get_listen_key(&api_key).await;
            },
            APIKeyEvent::Remove(APIKey::Binance(api_key)) => {
              if let Some(listen_key) = listen_keys.remove(&api_key.pub_key) {
                sockets.remove(&api_key.pub_key);
                let _ = me.close_listen_key(&api_key, &listen_key);
              }
            },
          }
          drop(me)
        },
        Some((listen_key, _)) = listen_key_sub.next() => {
          let me = me.lock().await;
          let socket = match me.init_websocket(
            format!("{}/{}", WS_ENDPOINT, listen_key.listen_key)
          ).await {
            Err(e) => {
              warn!(error = as_display!(e); "Switching Protocol Failed");
              continue;
            },
            Ok(v) => v,
          };
          drop(me);
          sockets.insert(listen_key.pub_key.clone(), socket);
          listen_keys.insert(listen_key.pub_key, listen_key.listen_key);
        },
        _ = listen_key_refresh.tick() => {
          let this = me.lock().await;
          let result_defer: Vec<_> = listen_keys
            .iter().filter_map(|(pub_key, lis_key)| {
              let header = this.pub_header_from_str(pub_key);
              return header.map(|header| (lis_key.to_string(), header)).ok();
            })
            .map(|(lis_key, header)| {
              let lis_key = lis_key.clone();
              let me = Arc::clone(&me);
              async move {
                let mut me = me.lock().await;
                return me.cli.put(
                  Some(header),
                  Some(&[("listenKey", lis_key)])
                ).await;
              }
            }).collect();
          join_all(result_defer).await;
          drop(this);
        },
        Some((pub_key, _)) = reauth_sub.next() => {
          let mut me = me.lock().await;
          match me.handle_disconnect(&pub_key).await {
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
          let me = me.lock().await;
          let user_data = match msg {
            Err(e) => {
              match e {
                WebSocketError::ConnectionClosed |
                WebSocketError::AlreadyClosed => {
                  sockets.remove(&api_key);
                  listen_keys.remove(&api_key);
                  let _ = me.reauth_pubsub.publish(&api_key);
                },
                _ => warn!(error = as_display!(e); "Failed to receive payload"),
              }
              continue;
            },
            Ok(v) => v,
          };
          me.handle_message(&api_key, &mut sockets, &mut listen_keys, &user_data).await?;
          drop(me);
        }
      };
    }
  }
}
