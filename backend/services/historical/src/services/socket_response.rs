use ::std::pin::Pin;
use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::futures::sink::{Sink, SinkExt};
use ::serde_json::to_string as jsonify;
use ::warp::ws::Message;
use ::warp::Error as WarpErr;

use ::rpc::progress::Progress;

use ::history::entities::FetchStatusChanged;

use crate::types::ProgressKVS;

pub struct SocketResponseService {
  pub num_obj: ProgressKVS,
  pub sync_prog: ProgressKVS,
}

impl SocketResponseService {
  pub fn new(num: ProgressKVS, sync: ProgressKVS) -> Self {
    Self {
      num_obj: num,
      sync_prog: sync,
    }
  }
}

#[async_trait]
pub trait ISocketResponseService {
  async fn handle(
    &self,
    item: &FetchStatusChanged,
    websink: Pin<Box<dyn Sink<Message, Error = WarpErr> + Send>>,
  );
}

#[async_trait]
impl ISocketResponseService for SocketResponseService {
  async fn handle(
    &self,
    item: &FetchStatusChanged,
    websink: Pin<Box<dyn Sink<Message, Error = WarpErr> + Send>>,
  ) {
    let exchange: Arc<String> = item.exchange.as_str().to_lowercase().into();
    let symbol: Arc<String> = item.symbol.into();
    let size = self
      .num_obj
      .get(exchange.clone(), symbol.clone())
      .await
      .unwrap_or(0);
    let cur = self
      .sync_prog
      .get(exchange.clone(), symbol.clone())
      .await
      .unwrap_or(0);
    let prog = Progress {
      exchange: Box::new(item.exchange),
      symbol: symbol.as_ref().clone(),
      size,
      cur,
    };
    let payload = jsonify(&prog)
      .unwrap_or(String::from("Failed to serialize the progress data."));
    let payload = Message::text(payload);
    let _ = websink.send(payload).await;
    let _ = websink.flush().await;
  }
}
