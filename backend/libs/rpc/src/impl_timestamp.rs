use ::errors::ParseError;
use ::types::chrono::{DateTime, TimeZone};
use ::types::DateTime as UTCDateTime;

use crate::timestamp::Timestamp;

impl<T> From<DateTime<T>> for Timestamp
where
  T: TimeZone,
{
  fn from(value: DateTime<T>) -> Self {
    return Self {
      secs: value.timestamp(),
      nanos: value.timestamp_subsec_nanos(),
    };
  }
}

impl TryFrom<Timestamp> for UTCDateTime {
  type Error = ParseError;
  fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
    return UTCDateTime::from_timestamp(value.secs, value.nanos).ok_or(
      ParseError::new(None, None, Some("Failed to parse timestamp")),
    );
  }
}
