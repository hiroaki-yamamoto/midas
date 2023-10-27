use ::std::marker::PhantomData;
use ::std::sync::Arc;

use ::std::convert::From;
use ::std::time::Duration;

use ::async_trait::async_trait;

use ::redis::{
  AsyncCommands as Commands, ExistenceCheck, RedisResult, SetExpiry, SetOptions,
};

use ::types::stateful_setter;

#[derive(Clone)]
pub struct WriteOption<C>
where
  C: Commands + Send + Sync,
{
  duration: Option<Duration>,
  non_existent_only: bool,
  _c: PhantomData<C>,
}

impl<C> Default for WriteOption<C>
where
  C: Commands + Send + Sync,
{
  fn default() -> Self {
    return Self {
      duration: Option::default(),
      non_existent_only: bool::default(),
      _c: PhantomData,
    };
  }
}

#[async_trait]
pub trait WriteOptionTrait {
  type Commands: Commands + Send + Sync;
  fn upcast(&self) -> Option<WriteOption<Self::Commands>>;
  fn duration(&self) -> Option<Duration>;
  fn non_existent_only(&self) -> bool;
  async fn execute(
    &self,
    mut cmds: Self::Commands,
    key: Arc<String>,
  ) -> RedisResult<()> {
    let mut res: RedisResult<()> = Ok(());
    if let Some(duration) = self.duration() {
      // let mut cmds = cmds.lock().await;
      res = res.and(
        cmds
          .pexpire(key.as_ref(), duration.as_millis() as usize)
          .await,
      );
    }
    return res;
  }
}

impl<C> From<WriteOption<C>> for SetOptions
where
  C: Commands + Send + Sync,
{
  fn from(value: WriteOption<C>) -> Self {
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

impl<C> WriteOption<C>
where
  C: Commands + Send + Sync,
{
  stateful_setter!(duration, Option<Duration>);
  stateful_setter!(non_existent_only, bool);
}

impl<C> WriteOptionTrait for WriteOption<C>
where
  C: Commands + Send + Sync,
{
  type Commands = C;
  fn upcast(&self) -> Option<Self> {
    return (*self).into();
  }
  fn duration(&self) -> Option<Duration> {
    return self.duration;
  }
  fn non_existent_only(&self) -> bool {
    return self.non_existent_only;
  }
}

#[async_trait]
impl<C> WriteOptionTrait for Option<WriteOption<C>>
where
  C: Commands + Send + Sync,
{
  type Commands = C;
  fn upcast(&self) -> Self {
    return *self;
  }
  fn duration(&self) -> Option<Duration> {
    return self.as_ref().and_then(|opt| opt.duration());
  }
  fn non_existent_only(&self) -> bool {
    return self
      .as_ref()
      .map(|opt| opt.non_existent_only())
      .unwrap_or(false);
  }
  async fn execute(
    &self,
    cmds: Self::Commands,
    key: Arc<String>,
  ) -> RedisResult<()> {
    if let Some(opt) = self {
      return opt.execute(cmds, key).await;
    }
    return Ok(());
  }
}
