mod filter;

use ::futures::stream::{iter, BoxStream, StreamExt};

use ::kvs::redis::{Commands, RedisError, RedisResult};
use ::kvs::traits::normal::Base;
use ::kvs_macros::last_check_kvs;
use ::rpc::entities::Exchanges;
use kvs::traits::normal::ChannelName;

pub use self::filter::NodeFilter;

last_check_kvs!(pub, ObserverNodeKVS, String, "observer_node:{}");
last_check_kvs!(pub, ONEXTypeKVS, String, "observer_node_exchange_type:{}");

impl<T> ONEXTypeKVS<T>
where
  T: Commands + Send + Sync,
{
  pub async fn get_nodes_by_exchange(
    &self,
    exchange: Exchanges,
  ) -> RedisResult<BoxStream<'_, String>> {
    let cmd_lock = self.commands();
    let keys: Vec<String> = async {
      let mut cmds = cmd_lock.lock().await;
      Ok::<Vec<String>, RedisError>(
        cmds.scan_match("observer_node_exchange_type:*")?.collect(),
      )
    }
    .await?;
    let keys = iter(keys)
      .map(move |key| {
        let exchange_key = key.clone();
        let cmd_lock = self.commands();
        let exchange = async move {
          let mut cmd = cmd_lock.lock().await;
          cmd.get::<_, String>(exchange_key.clone())
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

impl<T> ObserverNodeKVS<T>
where
  T: Commands + Send + Sync,
{
  /// Index node ids to KVS
  pub async fn index_node(&self, nodes: String) -> RedisResult<usize> {
    let channel_name = self.channel_name("node_index");
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    return cmd.sadd(channel_name, nodes);
  }
  pub async fn count_nodes(&self) -> RedisResult<usize> {
    let channel_name = self.channel_name("node_index");
    let cmd = self.commands();
    let mut cmd = cmd.lock().await;
    let node_count: usize = cmd
      .smembers::<_, String>(channel_name)
      .into_iter()
      .filter_map(|node_id| {
        let node_id = self.channel_name(&node_id);
        return cmd
          .lindex::<_, usize>(&node_id, 0)
          .map(move |_| node_id)
          .ok();
      })
      .count();
    return Ok(node_count);
  }
}
