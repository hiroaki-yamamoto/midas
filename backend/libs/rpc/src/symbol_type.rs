// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::symbol_type;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: symbol_type = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SymbolType {
    Crypto,
    Stock,
}
