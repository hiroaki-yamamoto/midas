use ::std::panic::UnwindSafe;
use ::std::sync::Mutex;

use ::slog::*;

fn new_root_logger<T>(drain: T) -> Logger
where
  T: SendSyncRefUnwindSafeDrain<Ok = (), Err = Never> + UnwindSafe + 'static,
{
  return Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));
}

pub fn build_debug() -> Logger {
  let dec = ::slog_term::TermDecorator::new().build();
  let drain = ::slog_term::FullFormat::new(dec).build().fuse();
  return new_root_logger(Mutex::new(drain).fuse());
}

pub fn build_json() -> Logger {
  let drain = ::slog_json::Json::new(::std::io::stdout())
    .add_default_keys()
    .build()
    .fuse();
  return new_root_logger(Mutex::new(drain).fuse());
}
