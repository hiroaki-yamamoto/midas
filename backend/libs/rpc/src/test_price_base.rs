// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::test_price_base;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: test_price_base = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TestPriceBase {
    Close,
    High,
    #[serde(rename = "highLowMid")]
    HighLowMid,
    Low,
    Open,
    #[serde(rename = "openCloseMid")]
    OpenCloseMid,
}
