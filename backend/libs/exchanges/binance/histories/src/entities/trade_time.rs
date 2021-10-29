use ::mongodb::bson::DateTime as MongoDateTime;
use ::std::time::SystemTime;
use serde::{Deserialize, Serialize};

use super::Kline;

use ::trade_observer::TradeDateTime;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeTime<T> {
  #[serde(rename = "_id")]
  pub symbol: String,
  pub open_time: T,
  pub close_time: T,
}

impl TradeTime<SystemTime> {
  fn from<S>(from: S) -> Self
  where
    S: TradeDateTime,
  {
    return Self {
      symbol: from.symbol(),
      open_time: from.open_time(),
      close_time: from.close_time(),
    };
  }
}

impl TradeTime<MongoDateTime> {
  fn from<T>(from: T) -> Self
  where
    T: TradeDateTime,
  {
    return Self {
      symbol: from.symbol(),
      open_time: from.open_time().into(),
      close_time: from.close_time().into(),
    };
  }
}

impl TradeDateTime for TradeTime<SystemTime> {
  fn open_time(&self) -> SystemTime {
    return self.open_time;
  }
  fn close_time(&self) -> SystemTime {
    return self.close_time;
  }
  fn symbol(&self) -> String {
    return self.symbol.clone();
  }
}

impl TradeDateTime for TradeTime<MongoDateTime> {
  fn open_time(&self) -> SystemTime {
    return self.open_time.into();
  }
  fn close_time(&self) -> SystemTime {
    return self.close_time.into();
  }
  fn symbol(&self) -> String {
    return self.symbol.clone();
  }
}

impl From<Kline> for TradeTime<SystemTime> {
  fn from(kline: Kline) -> Self {
    return Self::from(kline);
  }
}

impl From<&Kline> for TradeTime<SystemTime> {
  fn from(kline: &Kline) -> Self {
    return Self::from(kline.clone());
  }
}

impl From<Kline> for TradeTime<MongoDateTime> {
  fn from(kline: Kline) -> Self {
    return Self::from(kline);
  }
}

impl From<&Kline> for TradeTime<MongoDateTime> {
  fn from(kline: &Kline) -> Self {
    return Self::from(kline.clone());
  }
}

impl From<TradeTime<MongoDateTime>> for TradeTime<SystemTime> {
  fn from(mongo: TradeTime<MongoDateTime>) -> Self {
    return Self::from(mongo);
  }
}

impl From<&TradeTime<MongoDateTime>> for TradeTime<SystemTime> {
  fn from(mongo: &TradeTime<MongoDateTime>) -> Self {
    return Self::from(mongo.clone());
  }
}

impl From<TradeTime<SystemTime>> for TradeTime<MongoDateTime> {
  fn from(chrono_based: TradeTime<SystemTime>) -> Self {
    return Self::from(chrono_based);
  }
}

impl From<&TradeTime<SystemTime>> for TradeTime<MongoDateTime> {
  fn from(chrono_based: &TradeTime<SystemTime>) -> Self {
    return Self::from(chrono_based.clone());
  }
}
