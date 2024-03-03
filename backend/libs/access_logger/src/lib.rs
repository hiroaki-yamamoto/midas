mod entities;

use self::entities::Log as LogEntity;
use ::log::info;
use ::warp::log::{custom, Info, Log};

pub fn log() -> Log<impl Fn(Info<'_>) + Clone> {
  return custom(move |info| {
    let log: LogEntity = info.into();
    info!(info: serde = log; "Access log");
  });
}
