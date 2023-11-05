// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::bookticker;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: bookticker = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bookticker {
    pub ask_price: String,
    pub ask_qty: String,
    pub bid_price: String,
    pub bid_qty: String,
    pub id: String,
    pub symbol: String,
}
