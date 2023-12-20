use ::std::sync::Arc;

use ::futures::{SinkExt, StreamExt};
use ::rand::Rng;
use ::serde::Serialize;
use ::serde_json::to_string as jsonify;
use ::tokio::sync::oneshot::{channel, Sender};
use ::warp::ws::Message;
use ::warp::Filter;

pub fn setup_server<T>(payloads: &[T]) -> (Sender<()>, u16)
where
  T: Serialize,
{
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

pub fn setup_ping_server<T>(payloads: &[T]) -> (Sender<()>, u16)
where
  T: Serialize,
{
  let (sig, sig_rx) = channel::<()>();
  let payloads: Arc<Vec<Message>> = Arc::new(
    payloads
      .iter()
      .map(|payload| Message::ping(jsonify(payload).unwrap()))
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
            let msg = websocket.next().await.unwrap().unwrap();
            assert!(msg.is_pong());
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
