#[derive(
  Clone, PartialEq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize,
)]
pub struct BotInfo {
  #[prost(string, tag = "1")]
  pub id: std::string::String,
  #[prost(enumeration = "Strategy", tag = "2")]
  pub strategy: i32,
  #[prost(string, tag = "3")]
  pub name: std::string::String,
  #[prost(string, tag = "4")]
  pub base_currency: std::string::String,
  #[prost(string, tag = "5")]
  pub desc: std::string::String,
  #[prost(string, tag = "6")]
  pub config: std::string::String,
}
#[derive(
  Clone, PartialEq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize,
)]
pub struct CurrentPosition {
  #[prost(string, tag = "1")]
  pub id: std::string::String,
  #[prost(string, tag = "2")]
  pub bot_id: std::string::String,
  #[prost(string, tag = "3")]
  pub symbol: std::string::String,
  #[prost(double, tag = "4")]
  pub trading_amount: f64,
  #[prost(double, tag = "5")]
  pub profit_amount: f64,
  #[prost(double, tag = "6")]
  pub profit_percent: f64,
}
#[derive(
  Clone, PartialEq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize,
)]
pub struct BotInfoList {
  #[prost(message, repeated, tag = "1")]
  pub bots: ::std::vec::Vec<BotInfo>,
}
#[derive(
  Clone, PartialEq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize,
)]
pub struct BotInfoListRequest {
  #[prost(int64, tag = "1")]
  pub offset: i64,
  #[prost(int64, tag = "2")]
  pub limit: i64,
}
#[derive(
  Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize)]
pub enum Strategy {
  Trailing = 0,
}
