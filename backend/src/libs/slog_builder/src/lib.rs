use ::slog::*;
use ::slog_atomic::*;

fn new_root_logger(
    drain: Box<dyn Drain<Err = Never, Ok = ()> + Send>
) -> (Logger, AtomicSwitchCtrl) {
    let drain = ::slog_async::Async::new(drain).build().fuse();
    let drain = AtomicSwitch::new(drain);
    let ctrl = drain.ctrl();
    return (
        Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION"))),
        ctrl
    );
}

pub fn build_debug() -> (Logger, AtomicSwitchCtrl) {
    let dec = ::slog_term::TermDecorator::new().build();
    let drain = ::slog_term::FullFormat::new(dec).build().fuse();
    return new_root_logger(Box::new(drain));
}

pub fn build_json() -> (Logger, AtomicSwitchCtrl) {
    let drain = ::slog_json::Json::new(::std::io::stdout()).build().fuse();
    return new_root_logger(Box::new(drain));
}
