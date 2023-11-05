// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::history_fetch_request;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: history_fetch_request = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryFetchRequest {
    pub end: Option<TimestampSchema>,
    pub exchange: ExchangeEntity,
    pub start: Option<TimestampSchema>,
    pub symbol: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimestampSchema {
    pub mils: i64,
    pub nanos: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExchangeEntity {
    Binance,
}
