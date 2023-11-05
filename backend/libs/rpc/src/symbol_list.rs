// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::symbol_list;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: symbol_list = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

pub type SymbolList = Vec<SymbolInfoSchema>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolInfoSchema {
    pub base: String,
    pub base_commission_precision: i64,
    pub base_precision: i64,
    pub exchange: ExchangeEntity,
    pub quote: String,
    pub quote_commission_precision: i64,
    pub quote_precision: i64,
    pub status: String,
    pub symbol: String,
    #[serde(rename = "type")]
    pub symbol_info_schema_type: SymbolTypeSchema,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExchangeEntity {
    Binance,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SymbolTypeSchema {
    Crypto,
    Stock,
}
