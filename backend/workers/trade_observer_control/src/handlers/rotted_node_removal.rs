use ::std::sync::Arc;
use ::std::time::Duration;

use ::futures::{join, try_join};
use ::kvs::redis::Commands;
use ::kvs::traits::last_checked::{FindBefore, Remove};
use ::kvs::Connection;
use ::subscribe::nats::Client as Nats;

use ::observers::kvs::{ONEXTypeKVS, ObserverNodeKVS};

use crate::errors::Result as ControlResult;

pub struct RemoveRotHandler<C>
where
  C: Commands + Send + Sync,
{
  kvs: Connection<C>,
  nats: Nats,
}

impl<C> RemoveRotHandler<C>
where
  C: Commands + Send + Sync,
{
  pub fn new(kvs: Connection<C>, nats: Nats) -> Self {
    Self { kvs, nats }
  }

  pub async fn handle(&self, rot_dur: Duration) -> ControlResult<()> {
    let observer_node_kvs = ObserverNodeKVS::new(self.kvs.clone().into());
    let type_kvs = ONEXTypeKVS::new(self.kvs.clone().into());
    let (rotted, rotted_type) = join!(
      observer_node_kvs.find_before(rot_dur),
      type_kvs.find_before(rot_dur)
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
    let _: (usize, usize) = try_join!(
      observer_node_kvs.del(rotted.as_slice()),
      type_kvs.del(rotted_type.as_slice())
    )?;
    return Ok(());
  }
}
