use ::std::sync::Arc;

use ::mongodb::Database;

use ::position::binance::services::{
  OrderResponseRepo, PositionConverter as BinancePosConv,
};
use ::position::interfaces::{IPositionConverter, IPositionRepo};
use ::position::services::PositionRepo;
use ::rpc::exchanges::Exchanges;

#[cfg(debug_assertions)]
use ::position::services::{
  PositionDemoConverter as DemoPosConv, PositionDemoRepo,
};

pub struct Context {
  pub position_repo: Arc<dyn IPositionRepo + Send + Sync>,
  pub position_converter: Arc<dyn IPositionConverter + Send + Sync>,
}

impl Context {
  #[cfg(debug_assertions)]
  pub async fn new(demo_mode: bool, exchange: Exchanges, db: Database) -> Self {
    return if demo_mode {
      Self {
        position_repo: Arc::new(PositionDemoRepo::new()),
        position_converter: Arc::new(DemoPosConv::new()),
      }
    } else {
      Self {
        position_repo: Arc::new(PositionRepo::new(db.clone()).await),
        position_converter: match exchange {
          Exchanges::Binance => {
            let order_resp_repo =
              Arc::new(OrderResponseRepo::new(db.clone()).await);
            Arc::new(BinancePosConv::new(order_resp_repo))
          }
        },
      }
    };
  }
  #[cfg(not(debug_assertions))]
  pub async fn new(exchange: Exchanges, db: Database) -> Self {
    Self {
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
