use ::std::collections::HashSet;

use ::serde::{Deserialize, Serialize};

use ::rpc::entities::SymbolInfo;

use super::Filters;

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
      status: self.status,
    };
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolUpdateEvent {
  pub to_add: Vec<Symbol>,
  pub to_remove: Vec<Symbol>,
}

impl SymbolUpdateEvent {
  pub fn new<S, T>(new: S, old: T) -> Self
  where
    S: IntoIterator<Item = Symbol> + Clone,
    T: IntoIterator<Item = Symbol> + Clone,
  {
    let new_keys: HashSet<String> = new.clone().into_iter().map(|item| item.symbol).collect();
    let old_keys: HashSet<String> = old.clone().into_iter().map(|item| item.symbol).collect();

    let to_add: Vec<String> = (&new_keys - &old_keys).into_iter().collect();
    let to_add = new.into_iter().filter(move |item| { to_add.contains(&item.symbol) } ).collect();

    let to_remove: Vec<String> = (&old_keys - &new_keys).into_iter().collect();
    let to_remove = old.into_iter().filter(move |item| { to_remove.contains(&item.symbol) }).collect();
    return Self {to_add,  to_remove};
  }

  pub fn has_diff(&self) -> bool {
    return !self.to_add.is_empty() || !self.to_remove.is_empty();
  }
}
