mod filter;

use ::futures::stream::{iter, BoxStream, StreamExt};

use ::errors::KVSResult;
use ::kvs::redis::Commands;
use ::kvs::traits::normal::Base;
use ::kvs::traits::normal::ChannelName;
use ::kvs_macros::{kvs, last_check_kvs};
use ::rpc::entities::Exchanges;

pub use self::filter::NodeFilter;

last_check_kvs!(pub, ObserverNodeKVS, String, "observer_node:{}");
last_check_kvs!(pub, ONEXTypeKVS, String, "observer_node_exchange_type:{}");
kvs!(pub, InitLock, String, "init_lock:{}");

impl<T> ONEXTypeKVS<T>
where
  T: Commands + Send + Sync,
{
  /// Index node ids to KVS
  pub async fn index_node(&self, node: String) -> KVSResult<usize> {
    let channel_name = self.channel_name("node_index");
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    return Ok(cmd.sadd(channel_name, node)?);
  }

  /// Unindex node ids to KVS
  pub async fn unindex_node(&self, node: String) -> KVSResult<usize> {
    let channel_name = self.channel_name("node_index");
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    return Ok(cmd.srem(channel_name, node)?);
  }

  pub async fn get_nodes_by_exchange(
    &self,
    exchange: Exchanges,
  ) -> KVSResult<BoxStream<'_, String>> {
    let cmd = self.commands();
    let index_channel_name = self.channel_name("node_index");
    let keys: Vec<String> = async {
      let mut cmds = cmd.lock().await;
      cmds.smembers::<_, Vec<String>>(index_channel_name)
    }
    .await?;

    let keys = iter(keys)
      .map(move |key| {
        let exchange_key = self.channel_name(&key);
        let cmd = self.commands();
        let exchange = async move {
          let mut cmd = cmd.lock().await;
          cmd.get::<_, String>(exchange_key)
        };
        return (key, exchange);
      })
      .filter_map(|(key, exchange)| async {
        let pair = exchange
          .await
          .map(|exchange| (key, Exchanges::from_str_name(&exchange)));
        return pair.ok();
      })
      .filter_map(|(key, node_exchange)| async move {
        return node_exchange.map(|node_exchange| (key, node_exchange));
      })
      .filter_map(move |(key, node_exchange)| async move {
        if node_exchange == exchange {
          return Some(key);
        } else {
          return None;
        }
      });
    return Ok(keys.boxed());
  }
}

impl<T> ObserverNodeKVS<T> where T: Commands + Send + Sync {}
