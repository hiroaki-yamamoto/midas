use ::serde::{Deserialize, Serialize};

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
  #[serde(rename = "PERCENT_PRICE_BY_SIDE", rename_all = "camelCase")]
  PrecentPriceBySide {
    bid_multiplier_up: String,
    bid_multiplier_down: String,
    ask_multiplier_up: String,
    ask_multiplier_down: String,
    avg_price_mins: u32,
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
  #[serde(rename = "TRAILING_DELTA", rename_all = "camelCase")]
  TrailingDelta {
    min_trailing_above_delta: u32,
    max_trailing_above_delta: u32,
    min_trailing_below_delta: u32,
    max_trailing_below_delta: u32,
  },
}
