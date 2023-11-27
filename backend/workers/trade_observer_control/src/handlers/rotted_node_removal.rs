use ::std::sync::Arc;
use ::std::time::Duration;

use ::futures::future::{try_join, try_join_all, BoxFuture, FutureExt};
use ::futures::join;
use ::kvs::redis::aio::MultiplexedConnection;
use ::kvs::traits::last_checked::{FindBefore, Get, ListOp, Remove};
use ::subscribe::nats::Client as Nats;

use ::observers::entities::TradeObserverControlEvent;
use ::observers::kvs::{NODE_EXCHANGE_TYPE_KVS_BUILDER, NODE_KVS_BUILDER};
use ::observers::pubsub::NodeControlEventPubSub;
use ::rpc::exchanges::Exchanges;
use ::subscribe::PubSub;

use crate::errors::{Error as ControlError, Result as ControlResult};

pub struct RemoveRotHandler {
  node_kvs_listop: Arc<
    dyn ListOp<Commands = MultiplexedConnection, Value = String> + Send + Sync,
  >,
  node_kvs_find_before:
    Arc<dyn FindBefore<Commands = MultiplexedConnection> + Send + Sync>,
  node_kvs_del: Arc<dyn Remove<Commands = MultiplexedConnection> + Send + Sync>,
  type_kvs_get: Arc<
    dyn Get<Commands = MultiplexedConnection, Value = String> + Send + Sync,
  >,
  type_kvs_find_before:
    Arc<dyn FindBefore<Commands = MultiplexedConnection> + Send + Sync>,
  type_kvs_del: Arc<dyn Remove<Commands = MultiplexedConnection> + Send + Sync>,
  control_event: NodeControlEventPubSub,
}

impl RemoveRotHandler {
  pub async fn new(
    kvs: MultiplexedConnection,
    nats: &Nats,
  ) -> ControlResult<Self> {
    let node_kvs = Arc::new(NODE_KVS_BUILDER.build(kvs.clone()));
    let type_kvs = Arc::new(NODE_EXCHANGE_TYPE_KVS_BUILDER.build(kvs.clone()));
    return Ok(Self {
      node_kvs_listop: node_kvs.clone(),
      node_kvs_find_before: node_kvs.clone(),
      node_kvs_del: node_kvs.clone(),
      type_kvs_get: type_kvs.clone(),
      type_kvs_find_before: type_kvs.clone(),
      type_kvs_del: type_kvs.clone(),
      control_event: NodeControlEventPubSub::new(&nats).await?,
    });
  }

  pub fn publish_freed_symbols<'a>(
    &'a self,
    node_ids: &'a [Arc<String>],
  ) -> Vec<BoxFuture<'_, ControlResult<()>>> {
    let mut defers = vec![];
    for node_id in node_ids {
      let defer = async {
        let mut publish_defers = vec![];
        let exchange = self.type_kvs_get.get(node_id.clone()).await?;
        let exchange: Exchanges = exchange.parse()?;
        let symbols: Vec<String> =
          self.node_kvs_listop.lrange(node_id.clone(), 0, -1).await?;
        for symbol in symbols {
          let publish_defer =
            self
              .control_event
              .publish(TradeObserverControlEvent::SymbolAdd(
                Box::new(exchange),
                symbol,
              ));
          publish_defers.push(publish_defer)
        }
        let _ = try_join_all(publish_defers).await?;
        Ok::<_, ControlError>(())
      };
      defers.push(defer.boxed());
    }
    return defers;
  }

  pub async fn handle(&self, rot_dur: Duration) -> ControlResult<()> {
    let (rotted, rotted_type) = join!(
      self.node_kvs_find_before.find_before(rot_dur),
      self.type_kvs_find_before.find_before(rot_dur)
    );
    let rotted: Vec<Arc<String>> = rotted
      .unwrap_or(vec![])
      .into_iter()
      .map(|s| s.into())
      .collect();
    let rotted_type: Vec<Arc<String>> = rotted_type
      .unwrap_or(vec![])
      .into_iter()
      .map(|s| s.into())
      .collect();
    if rotted.is_empty() || rotted_type.is_empty() {
      return Ok(());
    }
    let _ = try_join_all(self.publish_freed_symbols(rotted.as_slice())).await?;
    let _: (usize, usize) = try_join(
      self.node_kvs_del.del(rotted.as_slice().into()),
      self.type_kvs_del.del(rotted_type.as_slice().into()),
    )
    .await?;
    return Ok(());
  }
}
