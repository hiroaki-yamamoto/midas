use ::std::collections::HashMap;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::futures::future::join_all;
use ::futures::{SinkExt, StreamExt};
use ::log::{as_display, error, info, warn};
use ::nats::jetstream::JetStream as Broker;
use ::serde_json::{from_slice as from_json_bin, from_str as from_json_str};
use ::tokio::select;
use ::tokio::time::{interval, sleep};
use ::tokio_stream::StreamMap;
use ::tokio_tungstenite::connect_async;
use ::tokio_tungstenite::tungstenite::{
  client::IntoClientRequest, Error as WebSocketError, Message,
};

use ::clients::binance::PubClient;
use ::clients::binance::{REST_ENDPOINT, WS_ENDPOINT};

use crate::traits::UserStream as UserStreamTrait;
use ::entities::{APIKey, APIKeyEvent, APIKeyInner};
use ::errors::{MaximumAttemptExceeded, WebsocketError};
use ::keychain::pubsub::APIKeyPubSub;
use ::subscribe::PubSub;
use ::types::{GenericResult, TLSWebSocket, ThreadSafeResult};

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
}

impl UserStream {
  pub fn new(broker: Broker) -> Self {
    return Self {
      key_pubsub: APIKeyPubSub::new(broker.clone()),
      notify_pubsub: NotifyPubSub::new(broker.clone()),
      reauth_pubsub: ReauthPubSub::new(broker.clone()),
      listen_key_pubsub: ListenKeyPubSub::new(broker.clone()),
    };
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
  async fn handle_user_stream_event(
    &self,
    uds: RawUserStreamEvents,
  ) -> GenericResult<()> {
    let _ = self.notify_pubsub.publish({
      let casted: GenericResult<CastedUserStreamEvents> = uds.into();
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
  ) -> GenericResult<()> {
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
    &self,
    pub_key: &String,
  ) -> Result<(), MaximumAttemptExceeded> {
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
    return Err(MaximumAttemptExceeded);
  }
}

impl PubClient for UserStream {}

#[async_trait]
impl UserStreamTrait for UserStream {
  async fn get_listen_key(
    &self,
    api_key: &APIKeyInner,
  ) -> ThreadSafeResult<()> {
    let pub_key = &api_key.pub_key;
    let client = self.get_client(pub_key)?;
    let resp: ListenKey = client
      .post(format!("{}/api/v3/userDataStream", REST_ENDPOINT).as_str())
      .send()
      .await?
      .error_for_status()?
      .json()
      .await?;
    let key = ListenKeyPair::new(resp.listen_key, pub_key.clone());
    let _ = self.listen_key_pubsub.publish(&key)?;
    return Ok(());
  }
  async fn close_listen_key(
    &self,
    api_key: &APIKeyInner,
    listen_key: &String,
  ) -> ThreadSafeResult<()> {
    let pub_key = &api_key.pub_key;
    let client = self.get_client(pub_key)?;
    let _ = client
      .delete(format!("{}/api/v3/userDataStream", REST_ENDPOINT).as_str())
      .query(&[("listenKey", listen_key)])
      .send()
      .await?
      .error_for_status()?;
    return Ok(());
  }
  async fn start(&self) -> GenericResult<()> {
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
    let me = self;
    loop {
      select! {
        Some((event, _)) = keychain_sub.next() => {
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
        },
        Some((listen_key, _)) = listen_key_sub.next() => {
          let socket = match me.init_websocket(
            format!("{}/{}", WS_ENDPOINT, listen_key.listen_key)
          ).await {
            Err(e) => {
              warn!(error = as_display!(e); "Switching Protocol Failed");
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
        Some((pub_key, _)) = reauth_sub.next() => {
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
        }
      };
    }
  }
}
