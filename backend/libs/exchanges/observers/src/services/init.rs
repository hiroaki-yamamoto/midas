use ::std::sync::Arc;

use ::futures::future::{try_join_all, FutureExt};

use ::config::Database;
use ::errors::{ObserverError, ObserverResult};
use ::kvs::redis::AsyncCommands as Commands;
use ::kvs::traits::normal::Lock;
use ::log::{as_serde, info};
use ::rpc::entities::Exchanges;
use ::subscribe::nats::Client as Nats;
use ::subscribe::PubSub;

use crate::kvs::INIT_LOCK_BUILDER;

use crate::pubsub::NodeControlEventPubSub;

use super::NodeDIffTaker;
use super::ObservationBalancer;

pub struct Init<C>
where
  C: Commands + Clone + Sync + Send,
{
  diff_taker: Arc<NodeDIffTaker<C>>,
  balancer: Arc<ObservationBalancer<C>>,
  control_pubsub: NodeControlEventPubSub,
  dlock: Arc<dyn Lock<Commands = C, Value = ObserverResult<()>> + Send + Sync>,
}

impl<C> Init<C>
where
  C: Commands + Clone + Sync + Send,
{
  pub async fn new(
    kvs: C,
    db: Database,
    nats: &Nats,
  ) -> ObserverResult<Init<C>> {
    let diff_taker =
      Arc::new(NodeDIffTaker::new(&db, kvs.clone().into()).await?);
    let balancer =
      Arc::new(ObservationBalancer::new(kvs.clone().into()).await?);
    let control_pubsub = NodeControlEventPubSub::new(nats).await?;
    let dlock = Arc::new(INIT_LOCK_BUILDER.build(kvs));

    return Ok(Self {
      diff_taker,
      balancer,
      control_pubsub,
      dlock,
    });
  }

  pub async fn init(&self, exchange: Exchanges) -> ObserverResult<()> {
    let diff_taker = self.diff_taker.clone();
    let balancer = self.balancer.clone();
    let control_pubsub = self.control_pubsub.clone();
    let _ = self
      .dlock
      .lock(exchange.as_str_name().to_string().into(), Arc::new(move || {
        async move {
          let exchange = exchange.clone();
          let diff = diff_taker.get_symbol_diff(&exchange).await?;
          let balanced = balancer.get_event_to_balancing(exchange).await?;
          let controls_to_publish = &diff | &balanced;
          info!(events = as_serde!(controls_to_publish); "Publishing symbol control events.");
          let defer: Vec<_> = controls_to_publish
            .into_iter()
            .map(|event| control_pubsub.publish(event))
            .collect();
          let _ = try_join_all(defer).await?;
          return Ok::<(), ObserverError>(());
        }.boxed()
      }))
      .await?;
    return Ok(());
  }
}
