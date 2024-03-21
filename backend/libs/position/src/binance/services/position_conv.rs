use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::futures::future::try_join;
use ::futures::StreamExt;
use ::rug::Float;

use ::errors::PositionResult;
use ::rpc::position::Position as PositionRpc;
use ::rpc::timestamp::Timestamp as RpcTimestamp;
use ::types::DateTime;

use crate::binance::interfaces::IOrderResponseRepo;
use crate::entities::Position;
use crate::interfaces::IPositionConverter;

pub struct PositionConverter {
  pub order_resp_repo: Arc<dyn IOrderResponseRepo>,
}

#[async_trait]
impl IPositionConverter for PositionConverter {
  async fn to_rpc(&self, position: &Position) -> PositionResult<PositionRpc> {
    let (entry_order_resp, exit_order_resp) = try_join(
      self.order_resp_repo.find_by_entry_position(position),
      self.order_resp_repo.find_by_exit_position(position),
    )
    .await?;
    let amount = entry_order_resp
      .filter_map(|order| async { order.ok() })
      .fold(Float::with_val(128, 0.0), |acc, res| async {
        acc + res.orig_qty.unwrap_or(Float::with_val(128, 0.0))
      })
      .await;
    let entry_at: DateTime = position.entry_at.into();
    let rpc_pos = PositionRpc {
      id: position.id.to_hex(),
      mode: Box::new(position.mode),
      bot_id: position.bot_id.to_hex(),
      symbol: position.symbol.clone(),
      entry_at: Box::new(entry_at.into()),
      exit_at: position.exit_at.map(|dt| {
        let dt: DateTime = dt.into();
        let dt: RpcTimestamp = dt.into();
        return Box::new(dt);
      }),
      amount: amount.to_string(),
    };
  }
}
