use crate::rpc::historical::{
  hist_chart_server::HistChart,
  HistChartProg, HistChartFetchReq, Status,
};

#[derive(Debug)]
pub struct Server {
}

impl HistChart for Server {}
