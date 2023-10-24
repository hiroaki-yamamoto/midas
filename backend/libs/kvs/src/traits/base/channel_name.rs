pub trait ChannelName {
  fn __channel_name__(&self, key: &str) -> String;
}
