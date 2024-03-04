use super::summary_detail::SummaryDetail;

#[derive(Debug, PartialEq, Clone, ::serde::Serialize, ::serde::Deserialize)]
pub struct BotGetRequest {
  #[serde(default="BotGetRequest::default_granularity")]
  pub granularity: Box<SummaryDetail>,
}
