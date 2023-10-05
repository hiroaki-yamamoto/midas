use ::std::sync::Arc;
use ::std::time::Duration;

use ::futures::future::{try_join, try_join_all, BoxFuture, FutureExt};
use ::futures::join;
use ::kvs::redis::Commands;
use ::kvs::traits::last_checked::{FindBefore, Get, ListOp, Remove};
use ::kvs::Connection;
use ::subscribe::nats::Client as Nats;

use ::entities::TradeObserverControlEvent;
use ::errors::UnknownExchangeError;
use ::observers::kvs::{ONEXTypeKVS, ObserverNodeKVS};
use ::observers::pubsub::NodeControlEventPubSub;
use ::rpc::entities::Exchanges;
use ::subscribe::PubSub;

use crate::errors::{Error as ControlError, Result as ControlResult};

pub struct RemoveRotHandler<C>
where
  C: Commands + Send + Sync,
{
  node_kvs: ObserverNodeKVS<C>,
  type_kvs: ONEXTypeKVS<C>,
  control_event: NodeControlEventPubSub,
}

impl<C> RemoveRotHandler<C>
where
  C: Commands + Send + Sync,
{
  pub async fn new(kvs: Connection<C>, nats: &Nats) -> ControlResult<Self> {
    return Ok(Self {
      node_kvs: ObserverNodeKVS::new(kvs.clone().into()),
      type_kvs: ONEXTypeKVS::new(kvs.clone().into()),
      control_event: NodeControlEventPubSub::new(&nats).await?,
    });
  }

  pub fn publish_freed_symbols<'a>(
    &'a self,
    node_ids: &'a [Arc<str>],
  ) -> Vec<BoxFuture<'_, ControlResult<()>>> {
    let mut defers = vec![];
    for node_id in node_ids {
      let defer = async {
        let mut publish_defers = vec![];
        let exchange = self.type_kvs.get(node_id).await?;
        let exchange: Exchanges = Exchanges::from_str_name(&exchange)
          .ok_or_else(|| UnknownExchangeError::new(exchange))?;
        let symbols: Vec<String> = self.node_kvs.lrange(node_id, 0, -1).await?;
        for symbol in symbols {
          let publish_defer = self
            .control_event
            .publish(TradeObserverControlEvent::SymbolAdd(exchange, symbol));
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
      self.node_kvs.find_before(rot_dur),
      self.type_kvs.find_before(rot_dur)
    );
    let rotted: Vec<Arc<str>> = rotted
      .unwrap_or(vec![])
      .into_iter()
      .map(|s| s.into())
      .collect();
    let rotted_type: Vec<Arc<str>> = rotted_type
      .unwrap_or(vec![])
      .into_iter()
      .map(|s| s.into())
      .collect();
    if rotted.is_empty() || rotted_type.is_empty() {
      return Ok(());
    }
    let _ = try_join_all(self.publish_freed_symbols(rotted.as_slice())).await?;
    let _: (usize, usize) = try_join(
      self.node_kvs.del(rotted.as_slice()),
      self.type_kvs.del(rotted_type.as_slice()),
    )
    .await?;
    return Ok(());
  }
}
