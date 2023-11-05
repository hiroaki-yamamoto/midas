// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::position;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: position = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub bot_id: Option<String>,
    pub id: Option<String>,
    pub status: Option<PositionStatusSchema>,
    pub symbol: Option<String>,
    pub trading_amount: Option<String>,
    pub validation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PositionStatusSchema {
    Closed,
    Open,
}
