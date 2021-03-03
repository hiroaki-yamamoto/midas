use ::mongodb::bson::DateTime as MongoDateTime;
use ::types::DateTime as ChronoDateTime;
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

impl TradeTime<ChronoDateTime> {
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

impl TradeDateTime for TradeTime<ChronoDateTime> {
  fn open_time(&self) -> ChronoDateTime {
    return self.open_time;
  }
  fn close_time(&self) -> ChronoDateTime {
    return self.close_time;
  }
  fn symbol(&self) -> String {
    return self.symbol.clone();
  }
}

impl TradeDateTime for TradeTime<MongoDateTime> {
  fn open_time(&self) -> ChronoDateTime {
    return *self.open_time;
  }
  fn close_time(&self) -> ChronoDateTime {
    return *self.close_time;
  }
  fn symbol(&self) -> String {
    return self.symbol.clone();
  }
}

impl From<Kline> for TradeTime<ChronoDateTime> {
  fn from(kline: Kline) -> Self {
    return Self::from(kline);
  }
}

impl From<&Kline> for TradeTime<ChronoDateTime> {
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

impl From<TradeTime<MongoDateTime>> for TradeTime<ChronoDateTime> {
  fn from(mongo: TradeTime<MongoDateTime>) -> Self {
    return Self::from(mongo);
  }
}

impl From<&TradeTime<MongoDateTime>> for TradeTime<ChronoDateTime> {
  fn from(mongo: &TradeTime<MongoDateTime>) -> Self {
    return Self::from(mongo.clone());
  }
}

impl From<TradeTime<ChronoDateTime>> for TradeTime<MongoDateTime> {
  fn from(chrono_based: TradeTime<ChronoDateTime>) -> Self {
    return Self::from(chrono_based);
  }
}

impl From<&TradeTime<ChronoDateTime>> for TradeTime<MongoDateTime> {
  fn from(chrono_based: &TradeTime<ChronoDateTime>) -> Self {
    return Self::from(chrono_based.clone());
  }
}
