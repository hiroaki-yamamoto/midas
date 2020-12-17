use ::serde::Serialize;

use ::exchanges::APIKey;

#[derive(Clone, Debug, Serialize)]
pub struct APIKeyList {
  pub keys: Vec<APIKey<String>>,
}
