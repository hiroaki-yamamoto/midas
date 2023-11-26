use ::errors::ParseError;
use ::types::DateTime as UTCDateTime;

use crate::timestamp::Timestamp;

impl From<UTCDateTime> for Timestamp {
  fn from(value: UTCDateTime) -> Self {
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
