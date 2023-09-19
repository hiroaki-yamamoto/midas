use ::serde::Deserialize;
use ::serde_json::Value as JSONValue;

#[derive(Deserialize, Debug, Clone)]
pub struct ResultValue {
  pub result: JSONValue,
  pub id: u64,
}
