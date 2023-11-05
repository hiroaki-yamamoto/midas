use super::timestamp::Timestamp;

use super::{
  bot::TimestampSchema as BotTS,
  history_fetch_request::TimestampSchema as HFRTS,
};

macro_rules! impl_cast {
    ($($name:ident),*) => {
        $(
            impl From<$name> for Timestamp {
                fn from(v: $name) -> Self {
                    return Self {
                        nanos: v.nanos,
                        mils: v.mils,
                    };
                }
            }

            impl From<Timestamp> for $name {
                fn from(value: Timestamp) -> Self {
                    return Self {
                        nanos: value.nanos,
                        mils: value.mils,
                    };
                }
            }
        )*
    };
}

impl_cast!(BotTS, HFRTS);
