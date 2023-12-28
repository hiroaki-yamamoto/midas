use ::std::fmt::Display;

use ::errors::ParseError;
use ::types::DateTime as UTCDateTime;

use crate::timestamp::Timestamp;

impl Display for Timestamp {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    return write!(f, "{}.{}", self.secs, self.nanos);
  }
}

impl From<UTCDateTime> for Timestamp {
  fn from(value: UTCDateTime) -> Self {
    return Self {
      secs: value.timestamp(),
      nanos: value.timestamp_subsec_nanos(),
    };
  }
}

impl TryFrom<&Timestamp> for UTCDateTime {
  type Error = ParseError;
  fn try_from(value: &Timestamp) -> Result<Self, Self::Error> {
    return UTCDateTime::from_timestamp(value.secs, value.nanos).ok_or(
      ParseError::new(None, None, Some("Failed to parse timestamp")),
    );
  }
}

impl TryFrom<Timestamp> for UTCDateTime {
  type Error = ParseError;
  fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
    return value.try_into();
  }
}
