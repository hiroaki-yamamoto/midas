use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::futures::future::try_join;
use ::futures::SinkExt;
use ::log::{as_serde, warn};
use ::serde_json::{from_reader as parse_json, to_string as jsonify};
use ::warp::ws::{Message, WebSocket};

use ::entities::HistoryFetchRequest;
use ::rpc::progress::Progress;
use ::subscribe::PubSub;

use crate::entities::SocketRequest;
use crate::errors::ServiceResult;
use crate::types::ProgressKVS;

pub struct SocketRequestService {
  splitter: Arc<dyn PubSub<Output = HistoryFetchRequest> + Send + Sync>,
  size: ProgressKVS,
  cur: ProgressKVS,
}

impl SocketRequestService {
  pub fn new(
    splitter: Arc<dyn PubSub<Output = HistoryFetchRequest> + Send + Sync>,
    size: ProgressKVS,
    cur: ProgressKVS,
  ) -> Self {
    return Self {
      splitter,
      size,
      cur,
    };
  }
}

#[async_trait]
pub trait ISocketRequestService {
  async fn handle(
    &self,
    msg: &Message,
    websink: &mut WebSocket,
  ) -> ServiceResult<()>;
}

#[async_trait]
impl ISocketRequestService for SocketRequestService {
  async fn handle(
    &self,
    msg: &Message,
    websink: &mut WebSocket,
  ) -> ServiceResult<()> {
    let req: SocketRequest = parse_json(msg.as_bytes())?;
    match req {
      SocketRequest::Fetch(req) => {
        let req: HistoryFetchRequest = req.into();
        match self.splitter.publish(&req).await {
          Ok(_) => {
            println!("Published Sync Start and End Date");
          }
          Err(e) => {
            println!("Publishing Sync Date Failed: {:?}", e);
          }
        }
      }
      SocketRequest::StatusCheck(req) => {
        let exchange = req.exchange.as_str().to_lowercase();
        let exchange = Arc::new(exchange);
        let symbol = Arc::new(req.symbol.clone());
        let (size, cur) = try_join(
          self.size.get(exchange.clone(), symbol.clone()),
          self.cur.get(exchange.clone(), symbol.clone()),
        )
        .await?;
        let size_cur = size.clone().zip(cur.clone());
        if let Some((size, cur)) = size_cur {
          let prog = Progress {
            exchange: req.exchange,
            symbol: symbol.as_ref().clone(),
            size,
            cur,
          };
          let payload = jsonify(&prog)
            .unwrap_or(String::from("Failed to serialize the progress data."));
          let payload = Message::text(payload);
          let mut websink = Box::pin(websink);
          let _ = websink.send(payload).await;
          let _ = websink.flush().await;
        } else {
          warn!(
            exchange = as_serde!(exchange),
            symbol = as_serde!(symbol),
            size = as_serde!(size),
            cur = as_serde!(cur);
            "Failed to get progress"
          );
        }
      }
    }
    return Ok(());
  }
}
