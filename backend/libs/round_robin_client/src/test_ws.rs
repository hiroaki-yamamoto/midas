use ::std::time::Duration;

use ::futures::StreamExt;
use ::serde::{Deserialize, Serialize};
use ::tokio::time::timeout;

use ::test_utils::websocket::{setup_ping_server, setup_server};

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

#[tokio::test]
async fn test_ping_pong() {
  let payloads = vec![
    SamplePayload::new("test1"),
    SamplePayload::new("test2"),
    SamplePayload::new("test3"),
  ];
  let correct = vec![
    MsgDetail::Continue,
    MsgDetail::Continue,
    MsgDetail::Continue,
  ];
  let (sig, port) = setup_ping_server(&payloads);
  let mut ws: WebSocket<SamplePayload, ()> =
    WebSocket::new(&[format!("ws://127.0.0.1:{}", port)])
      .await
      .unwrap();
  let mut received: Vec<MsgDetail<_>> = Vec::new();
  while let Some(payload) =
    timeout(Duration::from_secs(1), ws.next()).await.unwrap()
  {
    received.push(payload);
    if received.len() >= correct.len() {
      break;
    }
  }
  let _ = sig.send(());
  assert_eq!(received.len(), correct.len(), "received: {:?}", received);
  let mut pair = received.iter().zip(correct.iter());
  while let Some(pair) = pair.next() {
    assert_eq!(pair.0, pair.1);
  }
}
