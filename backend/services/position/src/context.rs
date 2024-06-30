use ::std::sync::Arc;

use ::mongodb::Database;

use ::position::binance::services::{
  OrderResponseRepo, PositionConverter as BinancePosConv,
};
use ::position::interfaces::{IPositionConverter, IPositionRepo};
use ::position::services::PositionRepo;
use ::rpc::exchanges::Exchanges;

#[cfg(debug_assertions)]
use ::position::services::PositionDemoRepo;

pub struct Context {
  #[cfg(debug_assertions)]
  pub position_demo_repo: Arc<dyn IPositionRepo + Send + Sync>,

  pub position_repo: Arc<dyn IPositionRepo + Send + Sync>,
  pub position_converter: Arc<dyn IPositionConverter + Send + Sync>,
}

impl Context {
  #[cfg(debug_assertions)]
  pub async fn new(exchange: Exchanges, db: Database) -> Self {
    return Self {
      position_demo_repo: Arc::new(PositionDemoRepo::new()),
      position_repo: Arc::new(PositionRepo::new(db.clone()).await),
      position_converter: match exchange {
        Exchanges::Binance => {
          let order_resp_repo =
            Arc::new(OrderResponseRepo::new(db.clone()).await);
          Arc::new(BinancePosConv::new(order_resp_repo))
        }
      },
    };
  }
  #[cfg(not(debug_assertions))]
  pub async fn new(exchange: Exchanges, db: Database) -> Self {
    return Self {
      position_repo: Arc::new(PositionRepo::new(db.clone()).await),
      position_converter: match exchange {
        Exchanges::Binance => {
          let order_resp_repo =
            Arc::new(OrderResponseRepo::new(db.clone()).await);
          Arc::new(BinancePosConv::new(order_resp_repo))
        }
      },
    };
  }
}
