use ::serde::{Deserialize, Serialize};
use ::serde_json::Value as JSONValue;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResultValue {
  pub result: JSONValue,
  pub id: String,
}
