use ::slog::*;
use ::slog_term::{CompactFormat, TermDecorator};

pub fn gen_drain() -> Fuse<::slog_atomic::AtomicSwitch> {
    let dec = TermDecorator::new().build();
    let drain = CompactFormat::new(dec).build().fuse();
    let drain = ::slog_async::Async::new(drain).chan_size(512).build().fuse();
    let drain = ::slog_atomic::AtomicSwitch::new(drain).fuse();
    return drain;
}
