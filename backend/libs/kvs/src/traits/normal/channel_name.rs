use ::std::fmt::Display;

pub trait ChannelName {
  fn channel_name(&self, key: impl AsRef<str> + Display) -> String;
}
