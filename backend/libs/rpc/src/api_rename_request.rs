// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::api_rename_request;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: api_rename_request = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiRenameRequest {
    pub label: String,
}
