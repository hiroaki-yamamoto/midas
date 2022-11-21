mod entities;

use self::entities::Log as LogEntity;
use ::log::{as_serde, info};
use ::warp::log::{custom, Info, Log};

pub fn log() -> Log<impl Fn(Info<'_>) + Clone> {
  return custom(move |info| {
    let log: LogEntity = info.into();
    info!(info = as_serde!(log); "Access log");
  });
}
