use ::std::sync::Arc;

pub trait ChannelName {
  fn __channel_name__(&self, key: Arc<String>) -> Arc<String>;
}
