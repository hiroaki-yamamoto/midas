use ::mongodb::bson::oid::ObjectId;

#[derive(Debug, Clone, Default)]
pub struct ExecutionResult {
  pub id: ObjectId,
  pub price: f64,
  pub qty: f64,
  pub profit: f64,
  pub profit_ratio: f64,
}
