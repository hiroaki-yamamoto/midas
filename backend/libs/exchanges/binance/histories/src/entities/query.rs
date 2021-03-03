use ::serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
  pub symbol: String,
  pub interval: String,
  pub start_time: String,
  pub end_time: Option<String>,
  pub limit: String,
}
