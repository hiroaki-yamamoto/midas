use ::chrono::offset::TimeZone;
use ::chrono::{DateTime, NaiveDateTime, Utc};
use ::mongodb::bson::{DateTime as BSONDateTime, Timestamp as BSONTimeStamp};

use super::timestamp::Timestamp;

impl From<BSONTimeStamp> for Timestamp {
  fn from(value: BSONTimeStamp) -> Self {
    return Self {
      mils: (value.time as i64) * 1000,
      nanos: 0,
    };
  }
}

impl From<Timestamp> for BSONTimeStamp {
  fn from(value: Timestamp) -> Self {
    return Self {
      time: (value.mils / 1000) as u32,
      increment: 0,
    };
  }
}

impl From<BSONDateTime> for Timestamp {
  fn from(value: BSONDateTime) -> Self {
    return Self {
      mils: value.timestamp_millis(),
      nanos: 0,
    };
  }
}

impl From<Timestamp> for BSONDateTime {
  fn from(value: Timestamp) -> Self {
    return Self::from_millis(value.mils);
  }
}

impl From<NaiveDateTime> for Timestamp {
  fn from(value: NaiveDateTime) -> Self {
    return Self {
      mils: value.timestamp_millis(),
      nanos: value.timestamp_subsec_nanos().into(),
    };
  }
}

impl From<Timestamp> for Option<NaiveDateTime> {
  fn from(value: Timestamp) -> Self {
    let secs = value.mils / 1000;
    let nanos: u32 = ((value.mils % 1000) * 1000000 + value.nanos)
      .try_into()
      .unwrap_or(0);
    return NaiveDateTime::from_timestamp_opt(secs, nanos);
  }
}

impl<Tz> From<DateTime<Tz>> for Timestamp
where
  Tz: TimeZone,
{
  fn from(value: DateTime<Tz>) -> Self {
    return Self {
      mils: value.timestamp_millis(),
      nanos: value.timestamp_subsec_nanos().into(),
    };
  }
}

impl From<Timestamp> for Option<DateTime<Utc>> {
  fn from(value: Timestamp) -> Self {
    let secs = value.mils / 1000;
    let nanos: u32 = ((value.mils % 1000) * 1000000 + value.nanos)
      .try_into()
      .unwrap_or(0);
    return DateTime::from_timestamp(secs, nanos);
  }
}
