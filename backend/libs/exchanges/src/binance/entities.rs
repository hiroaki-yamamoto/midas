use ::std::collections::HashSet;
use ::std::convert::AsRef;

use ::chrono::{DateTime as ChronoDateTime, Utc};
use ::mongodb::bson::DateTime as MongoDateTime;
use ::rpc::entities::SymbolInfo;
use ::serde::{Deserialize, Serialize};
use ::serde_json::Value;
use ::types::SendableErrorResult;

use crate::casting::{cast_datetime, cast_f64, cast_i64};
use crate::traits::TradeDateTime;

pub type BinancePayload = Vec<Vec<Value>>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistQuery {
  pub symbol: String,
  pub interval: String,
  pub start_time: String,
  pub end_time: Option<String>,
  pub limit: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
  pub timezone: String,
  pub exchange_filters: Vec<Value>,
  pub symbols: Vec<Symbol>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
  pub symbol: String,
  pub status: String,
  pub base_asset: String,
  pub base_asset_precision: i64,
  pub base_commission_precision: i64,
  pub quote_commission_precision: i64,
  pub quote_asset: String,
  pub quote_precision: i64,
  pub order_types: Vec<String>,
  pub oco_allowed: bool,
  pub iceberg_allowed: bool,
  pub quote_order_qty_market_allowed: bool,
  pub is_spot_trading_allowed: bool,
  pub is_margin_trading_allowed: bool,
  pub filters: Vec<Filters>,
  pub permissions: Vec<String>,
}

impl Symbol {
  pub fn as_symbol_info(self) -> SymbolInfo {
    return SymbolInfo {
      symbol: self.symbol,
      base: self.base_asset,
      quote: self.quote_asset,
    };
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType")]
pub enum Filters {
  #[serde(rename = "PRICE_FILTER", rename_all = "camelCase")]
  PriceFilter {
    min_price: String,
    max_price: String,
    tick_size: String,
  },
  #[serde(rename = "PERCENT_PRICE", rename_all = "camelCase")]
  PercentPrice {
    multiplier_up: String,
    multiplier_down: String,
    avg_price_mins: Option<f64>,
  },
  #[serde(rename = "LOT_SIZE", rename_all = "camelCase")]
  LotSize {
    min_qty: String,
    max_qty: String,
    step_size: String,
  },
  #[serde(rename = "MIN_NOTIONAL", rename_all = "camelCase")]
  MinNotional {
    min_notional: String,
    apply_to_market: bool,
    avg_price_mins: Option<f64>,
  },
  #[serde(rename = "ICEBERG_PARTS", rename_all = "camelCase")]
  IcebergParts { limit: Option<i32> },
  #[serde(rename = "MAX_NUM_ORDERS", rename_all = "camelCase")]
  MaxNumOrders { limit: Option<i32> },
  #[serde(rename = "MAX_NUM_ALGO_ORDERS", rename_all = "camelCase")]
  MaxNumAlgoOrders { max_num_algo_orders: Option<i32> },
  #[serde(rename = "MAX_NUM_ICEBERG_ORDERS", rename_all = "camelCase")]
  MaxNumIcebergOrders { max_num_iceberg_orders: i32 },
  #[serde(rename = "MAX_POSITION", rename_all = "camelCase")]
  MaxPosition { max_position: String },
  #[serde(rename = "MARKET_LOT_SIZE", rename_all = "camelCase")]
  MarketLotSize {
    min_qty: String,
    max_qty: String,
    step_size: String,
  },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistFetcherParam {
  pub symbol: String,
  pub num_symbols: i64,
  pub entire_data_len: i64,
  pub start_time: MongoDateTime,
  pub end_time: Option<MongoDateTime>,
}

impl AsRef<HistFetcherParam> for HistFetcherParam {
  fn as_ref(&self) -> &Self {
    return self;
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kline {
  pub symbol: String,
  pub open_time: MongoDateTime,
  pub open_price: f64,
  pub high_price: f64,
  pub low_price: f64,
  pub close_price: f64,
  pub volume: f64,
  pub close_time: MongoDateTime,
  pub quote_volume: f64,
  pub num_trades: i64,
  pub taker_buy_base_volume: f64,
  pub taker_buy_quote_volume: f64,
}

impl Kline {
  pub fn new(
    symbol: String,
    payload: &Vec<Value>,
  ) -> SendableErrorResult<Self> {
    return Ok(Kline {
      symbol,
      open_time: (cast_datetime("open_time", &payload[0])?).into(),
      open_price: cast_f64("open_price", &payload[1])?,
      high_price: cast_f64("high_price", &payload[2])?,
      low_price: cast_f64("low_price", &payload[3])?,
      close_price: cast_f64("close_price", &payload[4])?,
      volume: cast_f64("volume", &payload[5])?,
      close_time: (cast_datetime("close_time", &payload[6])?).into(),
      quote_volume: cast_f64("quote_volume", &payload[7])?,
      num_trades: cast_i64("num_trades", &payload[8])?,
      taker_buy_base_volume: cast_f64("taker_buy_base_volume", &payload[9])?,
      taker_buy_quote_volume: cast_f64("taker_buy_quote_volume", &payload[10])?,
    });
  }
}

pub type Klines = Vec<Kline>;

impl AsRef<Kline> for Kline {
  fn as_ref(&self) -> &Self {
    return self;
  }
}

impl TradeDateTime for Kline {
  fn open_time(&self) -> ChronoDateTime<Utc> {
    return *self.open_time;
  }
  fn close_time(&self) -> ChronoDateTime<Utc> {
    return *self.close_time;
  }
  fn symbol(&self) -> String {
    return self.symbol.clone();
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KlinesWithInfo {
  pub symbol: String,
  pub num_symbols: i64,
  pub entire_data_len: i64,
  pub klines: Klines,
}

impl AsRef<KlinesWithInfo> for KlinesWithInfo {
  fn as_ref(&self) -> &Self {
    return self;
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LatestTradeTime<T> {
  #[serde(rename = "_id")]
  pub symbol: String,
  pub open_time: T,
  pub close_time: T,
}

impl LatestTradeTime<ChronoDateTime<Utc>> {
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

impl LatestTradeTime<MongoDateTime> {
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

impl TradeDateTime for LatestTradeTime<ChronoDateTime<Utc>> {
  fn open_time(&self) -> ChronoDateTime<Utc> {
    return self.open_time;
  }
  fn close_time(&self) -> ChronoDateTime<Utc> {
    return self.close_time;
  }
  fn symbol(&self) -> String {
    return self.symbol.clone();
  }
}

impl TradeDateTime for LatestTradeTime<MongoDateTime> {
  fn open_time(&self) -> ChronoDateTime<Utc> {
    return *self.open_time;
  }
  fn close_time(&self) -> ChronoDateTime<Utc> {
    return *self.close_time;
  }
  fn symbol(&self) -> String {
    return self.symbol.clone();
  }
}

impl From<Kline> for LatestTradeTime<ChronoDateTime<Utc>> {
  fn from(kline: Kline) -> Self {
    return Self::from(kline);
  }
}

impl From<&Kline> for LatestTradeTime<ChronoDateTime<Utc>> {
  fn from(kline: &Kline) -> Self {
    return Self::from(kline.clone());
  }
}

impl From<Kline> for LatestTradeTime<MongoDateTime> {
  fn from(kline: Kline) -> Self {
    return Self::from(kline);
  }
}

impl From<&Kline> for LatestTradeTime<MongoDateTime> {
  fn from(kline: &Kline) -> Self {
    return Self::from(kline.clone());
  }
}

impl From<LatestTradeTime<MongoDateTime>>
  for LatestTradeTime<ChronoDateTime<Utc>>
{
  fn from(mongo: LatestTradeTime<MongoDateTime>) -> Self {
    return Self::from(mongo);
  }
}

impl From<&LatestTradeTime<MongoDateTime>>
  for LatestTradeTime<ChronoDateTime<Utc>>
{
  fn from(mongo: &LatestTradeTime<MongoDateTime>) -> Self {
    return Self::from(mongo.clone());
  }
}

impl From<LatestTradeTime<ChronoDateTime<Utc>>>
  for LatestTradeTime<MongoDateTime>
{
  fn from(chrono_based: LatestTradeTime<ChronoDateTime<Utc>>) -> Self {
    return Self::from(chrono_based);
  }
}

impl From<&LatestTradeTime<ChronoDateTime<Utc>>>
  for LatestTradeTime<MongoDateTime>
{
  fn from(chrono_based: &LatestTradeTime<ChronoDateTime<Utc>>) -> Self {
    return Self::from(chrono_based.clone());
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolUpdateEvent {
  pub to_add: Vec<String>,
  pub to_remove: Vec<String>,
}

impl SymbolUpdateEvent {
  pub fn new<S, T>(new: S, old: T) -> Self
  where
    S: IntoIterator<Item = String>,
    T: IntoIterator<Item = String>,
  {
    let new: HashSet<String> = new.into_iter().collect();
    let old: HashSet<String> = old.into_iter().collect();
    return Self {
      to_add: (&new - &old).into_iter().collect(),
      to_remove: (&old - &new).into_iter().collect(),
    };
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TradeSubRequestInner {
  pub id: u32,
  pub params: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "method")]
pub(crate) enum TradeSubRequest {
  #[serde(rename = "SUBSCRIBE")]
  Subscribe(TradeSubRequestInner),
  #[serde(rename = "UNSUBSCRIBE")]
  Unsubscribe(TradeSubRequestInner),
}
