// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::api_key_list;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: api_key_list = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiKeyList {
    pub keys: Option<Vec<ApiKeySchema>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiKeySchema {
    pub exchange: ExchangeEntity,
    pub id: Option<String>,
    pub label: String,
    pub prv: String,
    #[serde(rename = "pub")]
    pub api_key_schema_pub: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExchangeEntity {
    Binance,
}
