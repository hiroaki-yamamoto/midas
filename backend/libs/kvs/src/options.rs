use ::std::convert::From;
use ::std::time::Duration;

use ::async_trait::async_trait;
use ::redis::{ExistenceCheck, SetExpiry, SetOptions};
use ::types::stateful_setter;

#[derive(Clone)]
pub struct WriteOption {
  duration: Option<Duration>,
  non_existent_only: bool,
}

impl Default for WriteOption {
  fn default() -> Self {
    return Self {
      duration: Option::default(),
      non_existent_only: bool::default(),
    };
  }
}

#[async_trait]
pub trait WriteOptionTrait {
  fn duration(&self) -> Option<Duration>;
  fn non_existent_only(&self) -> bool;
}

impl From<WriteOption> for SetOptions {
  fn from(value: WriteOption) -> Self {
    let mut opt = SetOptions::default();
    if let Some(duration) = value.duration {
      opt = opt.with_expiration(SetExpiry::PX(duration.as_millis() as usize));
    }
    if value.non_existent_only {
      opt = opt.conditional_set(ExistenceCheck::NX);
    }
    return opt;
  }
}

impl WriteOption {
  stateful_setter!(duration, Option<Duration>);
  stateful_setter!(non_existent_only, bool);
}

impl WriteOptionTrait for WriteOption {
  fn duration(&self) -> Option<Duration> {
    return self.duration;
  }
  fn non_existent_only(&self) -> bool {
    return self.non_existent_only;
  }
}

#[async_trait]
impl WriteOptionTrait for Option<WriteOption> {
  fn duration(&self) -> Option<Duration> {
    return self.as_ref().and_then(|opt| opt.duration());
  }
  fn non_existent_only(&self) -> bool {
    return self
      .as_ref()
      .map(|opt| opt.non_existent_only())
      .unwrap_or(false);
  }
}
