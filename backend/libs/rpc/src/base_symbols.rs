// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::base_symbols;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: base_symbols = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

pub type BaseSymbols = Vec<String>;
