use ::futures::StreamExt;
use ::log::info;

use ::config::Database;
use ::config::ObserverConfig;
use ::errors::ObserverResult;
use ::kvs::redis::Commands;
use ::kvs::traits::normal::Lock;
use ::kvs::Connection;
use ::rpc::entities::Exchanges;
use ::subscribe::nats::Client as Nats;

use crate::kvs::InitLock;
use crate::kvs::ONEXTypeKVS;

use super::ObservationBalancer;
use super::SymbolSyncService;

pub struct Init<C>
where
  C: Commands + Sync + Send,
{
  type_kvs: ONEXTypeKVS<C>,
  init_lock: InitLock<C>,
  cfg: ObserverConfig,
  sync: SymbolSyncService<C>,
  balancer: ObservationBalancer<C>,
}

impl<C> Init<C>
where
  C: Commands + Sync + Send,
{
  pub async fn new(
    cfg: ObserverConfig,
    kvs: Connection<C>,
    db: Database,
    nats: &Nats,
  ) -> ObserverResult<Self> {
    let type_kvs = ONEXTypeKVS::new(kvs.clone().into());
    let init_lock = InitLock::new(kvs.clone().into());
    let sync = SymbolSyncService::new(&db, kvs.clone().into(), nats).await?;
    let balancer =
      ObservationBalancer::new(kvs.clone().into(), nats.clone()).await?;
    return Ok(Self {
      cfg,
      type_kvs,
      init_lock,
      sync,
      balancer,
    });
  }

  async fn count_nodes(&self, exchange: Exchanges) -> ObserverResult<usize> {
    return Ok(
      self
        .type_kvs
        .get_nodes_by_exchange(exchange)
        .await?
        .count()
        .await,
    );
  }

  pub async fn init(&self, exchange: Exchanges) -> ObserverResult<()> {
    let node_count = self.count_nodes(exchange).await?;
    let min_node_init = self.cfg.min_node_init(exchange);
    info!(
      node_count = node_count,
      min_node_init = min_node_init;
      "Retrive number of nodes"
    );
    if node_count == min_node_init {
      let _ = self
        .init_lock
        .lock("observer_control_node_event_handler", || async {
          info!("Init Triggered");
          return self.sync.handle(&exchange).await;
        })
        .await?;
    } else if node_count > min_node_init {
      let _ = self.balancer.broadcast_equalization(exchange, 0).await;
    }
    return Ok(());
  }
}
