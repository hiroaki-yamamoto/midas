use crate::rpc::{historical::hist_chart_server::HistChart};

#[derive(Debug)]
pub struct Server {
}

impl HistChart for Server {}