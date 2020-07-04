use ::async_trait::async_trait;

use ::tokio::sync::mpsc;
use ::tonic::{Request, Response};

use ::types::Result;

use crate::rpc::historical::{
  hist_chart_server::HistChart,
  HistChartProg, HistChartFetchReq, Status,
};

#[derive(Debug)]
pub struct Server {
}

#[async_trait]
impl HistChart for Server {
  type syncStream = mpsc::Receiver<Result<HistChartProg>>;

  async fn sync(
    &self, req: Request<HistChartFetchReq>,
  ) -> Result<Response<Self::syncStream>> {

  }
}
