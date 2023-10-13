use ::std::sync::Arc;
use ::std::time::Duration;

use ::futures::future::try_join;
use ::log::{as_error, error, info};
use ::tokio::time::sleep;

use ::errors::{ObserverError, UnknownExchangeError};
use ::kvs::redis::Commands;
use ::kvs::traits::last_checked::Get;
use ::kvs::traits::last_checked::ListOp;
use ::kvs::traits::last_checked::Remove;
use ::kvs::traits::last_checked::Set;
use ::kvs::Connection;
use ::kvs::WriteOption;
use ::random::generate_random_txt;
use ::rpc::entities::Exchanges;

use ::errors::ObserverResult;

use crate::kvs::{ONEXTypeKVS, ObserverNodeKVS};

const NODE_ID_TXT_SIZE: usize = 64;

/// This struct manages the node id.
///
/// **Note**: This struct doesn't publish any events to Trade Observer Control.
#[derive(Debug)]
pub struct NodeIDManager<T>
where
  T: Commands + Send + Sync,
{
  node_kvs: ObserverNodeKVS<T>,
  exchange_type_kvs: ONEXTypeKVS<T>,
}

impl<T> NodeIDManager<T>
where
  T: Commands + Sync + Send,
{
  pub fn new(con: Connection<T>) -> Self {
    return Self {
      node_kvs: ObserverNodeKVS::new(con.clone().into()),
      exchange_type_kvs: ONEXTypeKVS::new(con),
    };
  }

  /// Register the node with generating random number.
  ///
  /// **Return Value**: The node id.
  pub async fn register(&self, exchange: Exchanges) -> ObserverResult<String> {
    let mut node_id = generate_random_txt(NODE_ID_TXT_SIZE);
    loop {
      match self.exchange_type_kvs.index_node(node_id.clone()).await {
        Ok(num) => {
          if num > 0 {
            info!(node_id = node_id.to_string(); "Node indexed");
            break;
          } else {
            node_id = generate_random_txt(NODE_ID_TXT_SIZE);
            continue;
          }
        }
        Err(e) => {
          error!(error = as_error!(e); "Failed to index node");
          sleep(Duration::from_secs(1)).await;
          continue;
        }
      }
    }
    self
      .exchange_type_kvs
      .set(
        &node_id,
        exchange.as_str_name().into(),
        WriteOption::default()
          .duration(Duration::from_secs(30).into())
          .non_existent_only(true)
          .into(),
      )
      .await?;
    info!(node_id = node_id; "Acquired NodeID");
    return Ok(node_id);
  }

  /// Unregister the node.
  ///
  /// **Return Value**: (exchange_type, symbols: Vec<String>)
  pub async fn unregist(
    &self,
    node_id: &str,
  ) -> ObserverResult<(Exchanges, Vec<String>)> {
    let symbols: Vec<String> = self.node_kvs.lrange(node_id, 0, -1).await?;
    let exchange: String = self.exchange_type_kvs.get(node_id).await?;
    let exchange = Exchanges::from_str_name(&exchange)
      .ok_or::<ObserverError>(UnknownExchangeError::new(exchange).into())?;
    let node_id: Arc<str> = node_id.to_string().into();
    let (_, _): (usize, usize) = try_join(
      self.node_kvs.del(&[node_id.clone()]),
      self.exchange_type_kvs.del(&[node_id]),
    )
    .await?;
    return Ok((exchange, symbols));
  }
}
