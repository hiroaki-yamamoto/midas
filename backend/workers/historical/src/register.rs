use std::collections::HashMap;

use history::traits::Kline;
use history::HistoryFetcher;
use rpc::entities::Exchanges;

pub struct Register<'a> {
  registry: HashMap<Exchanges, &'a dyn HistoryFetcher<Kline = dyn Kline>>,
}

impl Register {
  pub fn new() -> Self {}
}
