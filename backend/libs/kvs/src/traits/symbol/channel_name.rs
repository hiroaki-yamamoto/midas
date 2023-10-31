use ::std::sync::Arc;

pub trait ChannelName {
  fn channel_name(
    &self,
    exchange: Arc<String>,
    symbol: Arc<String>,
  ) -> Arc<String>;
}
