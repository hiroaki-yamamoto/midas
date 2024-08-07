use ::serde::{Deserialize, Serialize};

use ::rpc::exchanges::Exchanges;
use ::rpc::symbol_info::SymbolInfo;
use ::rpc::symbol_type::SymbolType;

use super::filters::Filters;

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

impl From<&Symbol> for SymbolInfo {
  fn from(symbol: &Symbol) -> Self {
    return symbol.clone().into();
  }
}

impl From<Symbol> for SymbolInfo {
  fn from(symbol: Symbol) -> Self {
    return Self {
      symbol_type: Box::new(SymbolType::Crypto),
      exchange: Box::new(Exchanges::Binance),
      symbol: symbol.symbol,
      base: symbol.base_asset,
      base_precision: symbol.base_asset_precision,
      base_commission_precision: symbol.base_commission_precision,
      quote: symbol.quote_asset,
      quote_precision: symbol.quote_precision,
      quote_commission_precision: symbol.quote_commission_precision,
      status: symbol.status,
    };
  }
}
