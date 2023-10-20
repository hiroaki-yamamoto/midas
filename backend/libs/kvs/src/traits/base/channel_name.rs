pub trait ChannelName {
  fn channel_name(&self, key: &str) -> String;
}
