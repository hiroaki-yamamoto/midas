pub mod bookticker;
pub mod bot;
pub mod entities;
mod entities_impl;
pub mod historical;
mod historical_impl;
pub mod keychain;
pub mod rejection_handler;
pub mod symbols;

use ::chrono::{NaiveDateTime, Utc};
use ::types::DateTime;

pub mod google {
  pub mod protobuf {
    include!("./google.protobuf.rs");

    impl From<crate::DateTime> for Timestamp {
      fn from(value: crate::DateTime) -> Self {
        Self {
          seconds: value.timestamp(),
          nanos: value.timestamp_subsec_nanos() as i32,
        }
      }
    }

    impl From<Timestamp> for crate::DateTime {
      fn from(value: Timestamp) -> Self {
        Self::from_utc(
          crate::NaiveDateTime::from_timestamp(
            value.seconds,
            value.nanos as u32,
          ),
          crate::Utc,
        )
      }
    }
  }
}
