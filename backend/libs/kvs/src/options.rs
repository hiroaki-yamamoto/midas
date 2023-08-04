use ::std::fmt::Display;
use ::std::time::Duration;

use ::redis::{Commands, RedisResult};

use ::types::stateful_setter;

#[derive(Default, Clone)]
pub struct WriteOption {
  pub duration: Option<Duration>,
}

pub trait WriteOptionTrait {
  fn duration(&self) -> Option<Duration>;
  fn execute<S, T>(&self, cmds: &mut T, key: S) -> RedisResult<()>
  where
    S: AsRef<str> + Display,
    T: Commands,
  {
    let key = key.as_ref();
    let mut res: RedisResult<()> = Ok(());
    if let Some(duration) = self.duration() {
      res = res.and(cmds.pexpire(key, duration.as_millis() as usize));
    }
    return res;
  }
}

impl WriteOption {
  stateful_setter!(duration, Option<Duration>);
}

impl WriteOptionTrait for WriteOption {
  fn duration(&self) -> Option<Duration> {
    return self.duration;
  }
}

impl WriteOptionTrait for Option<WriteOption> {
  fn duration(&self) -> Option<Duration> {
    return self.as_ref().and_then(|opt| opt.duration());
  }
  fn execute<S, T>(&self, cmds: &mut T, key: S) -> RedisResult<()>
  where
    S: AsRef<str> + Display,
    T: Commands,
  {
    if let Some(opt) = self {
      return opt.execute(cmds, key);
    }
    return Ok(());
  }
}
