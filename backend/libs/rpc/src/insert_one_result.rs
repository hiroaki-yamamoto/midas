// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::insert_one_result;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: insert_one_result = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsertOneResult {
    pub id: String,
}
