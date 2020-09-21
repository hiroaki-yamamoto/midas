use ::chrono::{DateTime as ChronoDateTime, Utc};
use ::mongodb::bson::DateTime as MongoDateTime;
use ::rpc::entities::SymbolInfo;
use ::serde::{Deserialize, Serialize};
use ::serde_json::Value;
use ::types::SendableErrorResult;

use crate::casting::{cast_datetime, cast_f64, cast_i64};

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

#[derive(Debug)]
pub struct HistFetcherParam {
  pub symbol: String,
  pub num_symbols: i64,
  pub entire_data_len: i64,
  pub start_time: ChronoDateTime<Utc>,
  pub end_time: ChronoDateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
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

pub type KlineResults = Vec<SendableErrorResult<Kline>>;

pub struct KlineResultsWithSymbol {
  pub symbol: String,
  pub num_symbols: i64,
  pub entire_data_len: i64,
  pub klines: KlineResults,
}
