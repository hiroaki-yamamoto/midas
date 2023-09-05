mod connection;
mod options;
pub mod traits;

pub use ::redis;

pub use crate::options::WriteOption;
pub use crate::traits::symbol::Incr as IncrementalStore;

pub use crate::connection::Connection;

pub use ::errors::{KVSError, KVSResult};
