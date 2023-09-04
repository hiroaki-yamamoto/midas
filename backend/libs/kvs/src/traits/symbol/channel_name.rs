use ::std::fmt::Display;

pub trait ChannelName {
  fn channel_name(
    &self,
    exchange: impl AsRef<str> + Display,
    symbol: impl AsRef<str> + Display,
  ) -> String;
}
