use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::futures::future::try_join;
use ::futures::sink::SinkExt;
use ::log::warn;
use ::serde_json::to_string as jsonify;
use ::warp::ws::{Message, WebSocket};

use ::rpc::progress::Progress;

use ::history::entities::FetchStatusChanged;

use crate::errors::ServiceResult;
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
    websink: &mut WebSocket,
  ) -> ServiceResult<()>;
}

#[async_trait]
impl ISocketResponseService for SocketResponseService {
  async fn handle(
    &self,
    item: &FetchStatusChanged,
    websink: &mut WebSocket,
  ) -> ServiceResult<()> {
    let exchange: Arc<String> = item.exchange.as_str().to_lowercase().into();
    let symbol: Arc<String> = Arc::new(item.symbol.clone());
    let (size, cur) = try_join(
      self.num_obj.get(exchange.clone(), symbol.clone()),
      self.sync_prog.get(exchange.clone(), symbol.clone()),
    )
    .await?;
    let size_cur = size.clone().zip(cur.clone());
    if let Some((size, cur)) = size_cur {
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
    } else {
      warn!(
        exchange: serde = exchange,
        symbol: serde = symbol,
        size: serde = size,
        cur: serde = cur;
        "Failed to get progress"
      );
    }
    return Ok(());
  }
}
