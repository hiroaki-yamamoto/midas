use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::futures::future::try_join;
use ::futures::StreamExt;
use ::rug::Float;

use ::entities::{Order, OrderInner};
use ::errors::PositionResult;
use ::rpc::position::Position as PositionRpc;
use ::rpc::position_status::PositionStatus as RpcPositionStatus;
use ::rpc::timestamp::Timestamp as RpcTimestamp;
use ::types::DateTime;

use crate::binance::interfaces::IOrderResponseRepo;
use crate::entities::Position;
use crate::interfaces::IPositionConverter;

pub struct PositionConverter {
  pub order_resp_repo: Arc<dyn IOrderResponseRepo + Send + Sync>,
}

#[async_trait]
impl IPositionConverter for PositionConverter {
  async fn to_rpc(&self, position: &Position) -> PositionResult<PositionRpc> {
    let (entry_order_resp, exit_order_resp) = try_join(
      self.order_resp_repo.find_by_entry_position(position),
      self.order_resp_repo.find_by_exit_position(position),
    )
    .await?;
    let (amount, entry_order_inner) = entry_order_resp
      .filter_map(|order| async { order.ok() })
      .fold(
        (Float::with_val(128, 0.0), OrderInner::default()),
        |acc, res| async {
          let order: Order = (&res).into();
          return (
            acc.0
              + res.orig_qty.unwrap_or(Float::with_val(128, 0.0))
                * res.price.unwrap_or(Float::with_val(128, 0.0)),
            acc.1 + order.sum(),
          );
        },
      )
      .await;
    let (exit_order_inner_len, exit_order_inner) = exit_order_resp
      .filter_map(|order| async { order.ok() })
      .fold(
        (0, OrderInner::default()),
        |(acc_order_len, acc_order_inner), res| async move {
          let order: Order = (&res).into();
          return (
            acc_order_len + order.inner.len(),
            acc_order_inner + order.sum(),
          );
        },
      )
      .await;
    let entry_at: DateTime = position.entry_at.into();
    let rpc_pos = PositionRpc {
      id: position.id.to_hex(),
      status: Box::new(if exit_order_inner_len < 1 {
        RpcPositionStatus::CLOSE
      } else {
        RpcPositionStatus::OPEN
      }),
      mode: Box::new(position.mode.clone()),
      bot_id: position.bot_id.to_hex(),
      symbol: position.symbol.clone(),
      entry_at: Box::new(entry_at.into()),
      exit_at: position.exit_at.map(|dt| {
        let dt: DateTime = dt.into();
        let dt: RpcTimestamp = dt.into();
        return Box::new(dt);
      }),
      amount: amount.to_string(),
      entry_price: entry_order_inner.price.to_string(),
      exit_price: if exit_order_inner_len > 0 {
        Some(exit_order_inner.price.to_string())
      } else {
        None
      },
    };
    return Ok(rpc_pos);
  }
}
