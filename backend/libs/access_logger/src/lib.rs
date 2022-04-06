mod entities;

use self::entities::Log as LogEntity;
use ::slog::Logger;
use ::warp::log::{custom, Info, Log};

pub fn log(logger: Logger) -> Log<impl Fn(Info<'_>) + Clone> {
  return custom(move |info| {
    let log: LogEntity = info.into();
    ::slog::info!(logger, "{}", log);
  });
}
