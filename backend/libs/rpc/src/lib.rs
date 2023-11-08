pub mod bookticker;
pub mod bot;
pub mod entities;
mod entities_impl;
pub mod historical;
pub mod keychain;
pub mod rejection_handler;
pub mod symbols;

use ::std::convert::TryFrom;
use ::std::time::{SystemTime as DateTime, SystemTimeError};

pub mod google {
  pub mod protobuf {
    use std::time::Duration;

    include!("./google.protobuf.rs");

    impl crate::TryFrom<crate::DateTime> for Timestamp {
      type Error = crate::SystemTimeError;
      fn try_from(value: crate::DateTime) -> Result<Self, Self::Error> {
        let timestamp = value.duration_since(crate::DateTime::UNIX_EPOCH)?;
        return Ok(Self {
          seconds: timestamp.as_secs() as i64,
          nanos: timestamp.subsec_nanos() as i32,
        });
      }
    }

    impl From<Timestamp> for crate::DateTime {
      fn from(value: Timestamp) -> Self {
        return Self::UNIX_EPOCH
          + Duration::new(value.seconds as u64, value.nanos as u32);
      }
    }
  }
}
