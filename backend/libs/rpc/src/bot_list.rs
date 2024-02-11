use super::bot_response::BotResponse;

#[derive(Debug, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotList {
  pub bots: Vec<Box<BotResponse>>,
}
