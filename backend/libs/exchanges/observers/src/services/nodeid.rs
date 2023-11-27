use ::std::fmt::Debug;
use ::std::marker::PhantomData;
use ::std::sync::Arc;
use ::std::time::Duration;

use ::futures::future::{try_join_all, FutureExt};
use ::log::{as_error, error, info, warn};
use ::tokio::time::sleep;

use ::kvs::redis::AsyncCommands as Commands;
use ::kvs::traits::last_checked::{Get, ListOp, Remove, Set};
use ::kvs::WriteOption;
use ::random::generate_random_txt;
use ::rpc::exchanges::Exchanges;

use ::errors::ObserverResult;

use super::NodeIndexer;
use crate::kvs::{NODE_EXCHANGE_TYPE_KVS_BUILDER, NODE_KVS_BUILDER};

const NODE_ID_TXT_SIZE: usize = 64;

type NodeKVSList<C> =
  Arc<dyn ListOp<Commands = C, Value = String> + Send + Sync>;
type NodeKVSDel<C> = Arc<dyn Remove<Commands = C> + Send + Sync>;

/// This struct manages the node id.
///
/// **Note**: This struct doesn't publish any events to Trade Observer Control.
pub struct NodeIDManager<T>
where
  T: Commands + Clone + Send + Sync + Debug + 'static,
{
  node_kvs_list: NodeKVSList<T>,
  node_kvs_del: NodeKVSDel<T>,
  indexer: Arc<NodeIndexer<T>>,
  exchange_type_kvs_set:
    Arc<dyn Set<Commands = T, Value = String> + Send + Sync>,
  exchange_type_kvs_get:
    Arc<dyn Get<Commands = T, Value = String> + Send + Sync>,
  exchange_type_kvs_del: Arc<dyn Remove<Commands = T> + Send + Sync>,
  _t: PhantomData<T>,
}

impl<T> NodeIDManager<T>
where
  T: Commands + Clone + Sync + Send + Debug + 'static,
{
  pub fn new(con: T) -> Self {
    let node_kvs: Arc<_> = NODE_KVS_BUILDER.build(con.clone()).into();
    let node_kvs_list: NodeKVSList<_> = node_kvs.clone();
    let node_kvs_del: NodeKVSDel<_> = node_kvs.clone();

    let exchange_type_kvs = Arc::new(NODE_EXCHANGE_TYPE_KVS_BUILDER.build(con));
    let indexer = Arc::new(NodeIndexer::new(exchange_type_kvs.clone()));

    let exchange_type_kvs_set = exchange_type_kvs.clone();
    let exchange_type_kvs_get = exchange_type_kvs.clone();
    let exchange_type_kvs_del = exchange_type_kvs.clone();
    return Self {
      node_kvs_list,
      node_kvs_del,
      indexer,
      exchange_type_kvs_set,
      exchange_type_kvs_get,
      exchange_type_kvs_del,
      _t: PhantomData,
    };
  }

  /// Register the node with generating random number.
  ///
  /// **Return Value**: The node id.
  pub async fn register(&self, exchange: Exchanges) -> ObserverResult<String> {
    let mut node_id = generate_random_txt(NODE_ID_TXT_SIZE);
    loop {
      match self.indexer.index_node(node_id.clone()).await {
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
      .exchange_type_kvs_set
      .set(
        node_id.clone().into(),
        exchange.as_str().into(),
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
    node_id: Arc<String>,
  ) -> ObserverResult<(Exchanges, Vec<String>)> {
    let symbols: Vec<String> =
      self.node_kvs_list.lrange(node_id.clone(), 0, -1).await?;
    let exchange: String =
      self.exchange_type_kvs_get.get(node_id.clone()).await?;
    let exchange = exchange.parse()?;
    let _: Vec<_> = try_join_all([
      self.node_kvs_del.del(Arc::new([node_id.clone()])),
      self.exchange_type_kvs_del.del(Arc::new([node_id.clone()])),
      self
        .indexer
        .unindex_node(node_id.clone().to_string())
        .boxed(),
    ])
    .await?;
    return Ok((exchange, symbols));
  }
}
