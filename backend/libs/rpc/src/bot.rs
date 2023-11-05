// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::bot;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: bot = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bot {
    pub base_currency: String,
    pub condition: String,
    pub created_at: Option<TimestampSchema>,
    pub exchange: ExchangeEntity,
    pub id: Option<String>,
    pub name: String,
    pub trading_amount: String,
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
