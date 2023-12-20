use ::std::sync::Arc;
use ::std::time::Duration;

use ::futures::{SinkExt, StreamExt};
use ::rand::Rng;
use ::serde::{Deserialize, Serialize};
use ::serde_json::to_string as jsonify;
use ::tokio::sync::oneshot::{channel, Sender};
use ::tokio::time::timeout;
use ::warp::ws::Message;
use ::warp::Filter;

// use crate::interfaces::IWebSocketStream;
use crate::entities::WSMessageDetail as MsgDetail;

use super::WebSocket;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct SamplePayload {
  pub name: String,
}

impl SamplePayload {
  pub fn new(name: &str) -> Self {
    return Self {
      name: name.to_string(),
    };
  }
}

fn setup_server(payloads: &[SamplePayload]) -> (Sender<()>, u16) {
  let (sig, sig_rx) = channel::<()>();
  let payloads: Arc<Vec<Message>> = Arc::new(
    payloads
      .iter()
      .map(|payload| Message::text(jsonify(payload).unwrap()))
      .collect(),
  );
  let route = ::warp::any()
    .and(::warp::ws())
    .map(move |ws: ::warp::ws::Ws| {
      let payloads = payloads.clone();
      return ws.on_upgrade(
        |mut websocket: ::warp::ws::WebSocket| async move {
          for payload in payloads.iter() {
            let _ = websocket.send(payload.clone()).await;
            let _ = websocket.flush().await;
          }
        },
      );
    });
  let port: u16 = {
    let mut rng = ::rand::thread_rng();
    rng.gen_range(10000..65535)
  };
  ::tokio::spawn(
    ::warp::serve(route)
      .bind_with_graceful_shutdown(([127, 0, 0, 1], port), async move {
        sig_rx.await.ok();
      })
      .1,
  );
  return (sig, port);
}

#[tokio::test]
async fn test_ws() {
  let payloads = vec![
    SamplePayload::new("test1"),
    SamplePayload::new("test2"),
    SamplePayload::new("test3"),
  ];
  let (sig, port) = setup_server(&payloads);
  let mut ws: WebSocket<SamplePayload, ()> =
    WebSocket::new(&[format!("ws://127.0.0.1:{}", port)])
      .await
      .unwrap();
  let mut received: Vec<SamplePayload> = Vec::new();
  while let Some(payload) =
    timeout(Duration::from_secs(1), ws.next()).await.unwrap()
  {
    if let MsgDetail::EntityReceived(payload) = payload {
      received.push(payload);
    } else {
      println!("payload: {:?}", payload);
      panic!("Unexpected payload");
    }
    if received.len() >= payloads.len() {
      break;
    }
  }
  let _ = sig.send(());
  assert_eq!(received, payloads);
}
