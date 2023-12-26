use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::futures::sink::SinkExt;
use ::serde_json::to_string as jsonify;
use ::warp::ws::{Message, WebSocket};

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
  async fn handle(&self, item: &FetchStatusChanged, websink: &mut WebSocket);
}

#[async_trait]
impl ISocketResponseService for SocketResponseService {
  async fn handle(&self, item: &FetchStatusChanged, websink: &mut WebSocket) {
    let exchange: Arc<String> = item.exchange.as_str().to_lowercase().into();
    let symbol: Arc<String> = Arc::new(item.symbol.clone());
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
