use ::slog::*;
use ::slog_term::{FullFormat, TermDecorator};

pub fn gen_drain() -> Fuse<::slog_atomic::AtomicSwitch> {
    let dec = TermDecorator::new().build();
    let drain = FullFormat::new(dec).build().fuse();
    let drain = ::slog_async::Async::new(drain).chan_size(512).overflow_strategy(
        slog_async::OverflowStrategy::Block,
    ).build().fuse();
    let drain = ::slog_atomic::AtomicSwitch::new(drain).fuse();
    return drain;
}
