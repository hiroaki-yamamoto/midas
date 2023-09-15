pub trait ChannelName {
  fn channel_name(&self, exchange: &str, symbol: &str) -> String;
}
