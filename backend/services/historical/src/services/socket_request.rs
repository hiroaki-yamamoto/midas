use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::serde_json::from_reader as parse_json;
use ::warp::ws::Message;

use ::entities::HistoryFetchRequest;
use ::rpc::history_fetch_request::HistoryFetchRequest as RPCHistFetchReq;
use ::subscribe::PubSub;

use crate::errors::ServiceResult;

pub struct SocketRequestService {
  splitter: Arc<dyn PubSub<Output = HistoryFetchRequest> + Send + Sync>,
}

impl SocketRequestService {
  pub fn new(
    splitter: Arc<dyn PubSub<Output = HistoryFetchRequest> + Send + Sync>,
  ) -> Self {
    return Self { splitter };
  }
}

#[async_trait]
pub trait ISocketRequestService {
  async fn handle(&self, msg: &Message) -> ServiceResult<()>;
}

#[async_trait]
impl ISocketRequestService for SocketRequestService {
  async fn handle(&self, msg: &Message) -> ServiceResult<()> {
    let req: RPCHistFetchReq = parse_json(msg.as_bytes())?;
    let req: HistoryFetchRequest = req.into();
    match self.splitter.publish(&req).await {
      Ok(_) => {
        println!("Published Sync Start and End Date");
      }
      Err(e) => {
        println!("Publishing Sync Date Failed: {:?}", e);
      }
    }
    return Ok(());
  }
}
