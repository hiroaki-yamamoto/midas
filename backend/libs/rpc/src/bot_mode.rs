
#[derive(Debug, PartialEq, Clone, ::serde::Serialize, ::serde::Deserialize)]
pub enum BotMode {
  BackTest,
  ForwardTest,
  Live,
}
