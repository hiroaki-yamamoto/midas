mod connection;
mod options;
mod structures;
pub mod traits;

pub use ::redis;

pub use crate::options::WriteOption;

pub use crate::connection::Connection;
pub use crate::structures::NormalKVS;

pub use ::errors::{KVSError, KVSResult};
