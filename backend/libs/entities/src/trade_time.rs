use mongodb::bson::DateTime as MongoDateTime;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub trait TradeTimeTrait {
  fn symbol(&self) -> String;
  fn open_time(&self) -> SystemTime;
  fn close_time(&self) -> SystemTime;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeTime<T> {
  #[serde(rename = "_id")]
  pub symbol: String,
  pub open_time: T,
  pub close_time: T,
}

impl TradeTime<SystemTime> {
  pub fn from<S>(from: S) -> Self
  where
    S: TradeTimeTrait,
  {
    return Self {
      symbol: from.symbol(),
      open_time: from.open_time(),
      close_time: from.close_time(),
    };
  }
}

impl TradeTime<MongoDateTime> {
  pub fn from<T>(from: T) -> Self
  where
    T: TradeTimeTrait,
  {
    return Self {
      symbol: from.symbol(),
      open_time: from.open_time().into(),
      close_time: from.close_time().into(),
    };
  }
}

impl TradeTimeTrait for TradeTime<SystemTime> {
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

impl TradeTimeTrait for TradeTime<MongoDateTime> {
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
  fn from(system_based: TradeTime<SystemTime>) -> Self {
    return Self::from(system_based);
  }
}

impl From<&TradeTime<SystemTime>> for TradeTime<MongoDateTime> {
  fn from(system_based: &TradeTime<SystemTime>) -> Self {
    return Self::from(system_based.clone());
  }
}
