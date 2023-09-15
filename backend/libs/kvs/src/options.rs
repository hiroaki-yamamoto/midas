use ::std::convert::From;
use ::std::time::Duration;

use ::redis::{Commands, ExistenceCheck, RedisResult, SetExpiry, SetOptions};

use ::types::stateful_setter;

#[derive(Default, Clone)]
pub struct WriteOption {
  duration: Option<Duration>,
  non_existent_only: bool,
}

pub trait WriteOptionTrait {
  fn duration(&self) -> Option<Duration>;
  fn non_existent_only(&self) -> bool;
  fn execute(&self, cmds: &mut impl Commands, key: &str) -> RedisResult<()> {
    let mut res: RedisResult<()> = Ok(());
    if let Some(duration) = self.duration() {
      res = res.and(cmds.pexpire(key, duration.as_millis() as usize));
    }
    return res;
  }
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
  fn execute(&self, cmds: &mut impl Commands, key: &str) -> RedisResult<()> {
    if let Some(opt) = self {
      return opt.execute(cmds, key);
    }
    return Ok(());
  }
}
