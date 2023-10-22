use ::std::sync::Arc;
use ::std::time::Duration;

use ::futures::future::{try_join_all, FutureExt};
use ::log::{as_error, error, info, warn};
use ::tokio::time::sleep;

use ::errors::{ObserverError, UnknownExchangeError};
use ::kvs::redis::AsyncCommands as Commands;
use ::kvs::traits::last_checked::{Get, ListOp, Remove, Set};
use ::kvs::WriteOption;
use ::random::generate_random_txt;
use ::rpc::entities::Exchanges;

use ::errors::ObserverResult;

use crate::kvs::{NODE_EXCHANGE_TYPE_KVS_BUILDER, NODE_KVS_BUILDER};

const NODE_ID_TXT_SIZE: usize = 64;

/// This struct manages the node id.
///
/// **Note**: This struct doesn't publish any events to Trade Observer Control.
#[derive(Debug)]
pub struct NodeIDManager<T>
where
  T: Commands + Send + Sync,
{
  node_kvs: Arc<dyn Remove<T> + ListOp<T, String>>,
  exchange_type_kvs: Arc<dyn Get<T, String> + Set<T, String> + Remove<T>>,
}

impl<T> NodeIDManager<T>
where
  T: Commands + Sync + Send,
{
  pub fn new(con: T) -> Self {
    return Self {
      node_kvs: NODE_KVS_BUILDER.build(con.clone()),
      exchange_type_kvs: NODE_EXCHANGE_TYPE_KVS_BUILDER.build(con),
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
            warn!(
              node_id = node_id.to_string();
              "Node ID already exists. Regenerating..."
            );
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
    let _: Vec<()> = try_join_all([
      self.node_kvs.del(&[node_id.clone()]),
      self.exchange_type_kvs.del(&[node_id.clone()]),
      async move {
        self
          .exchange_type_kvs
          .unindex_node(node_id.clone().to_string())
          .await?;
        Ok(())
      }
      .boxed(),
    ])
    .await?;
    return Ok((exchange, symbols));
  }
}
