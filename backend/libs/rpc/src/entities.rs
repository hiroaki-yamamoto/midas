#[derive(
  Clone, PartialEq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize,
)]
pub struct SymbolInfo {
  #[prost(string, tag = "1")]
  pub symbol: std::string::String,
  #[prost(string, tag = "2")]
  pub status: std::string::String,
  #[prost(string, tag = "3")]
  pub base: std::string::String,
  #[prost(string, tag = "4")]
  pub quote: std::string::String,
}
#[derive(
  Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
)]
#[repr(i32)]
#[derive(
  ::serde::Serialize,
  ::serde::Deserialize,
  ::num_derive::FromPrimitive,
  ::clap::Clap,
)]
#[serde(tag = "exchange")]
pub enum Exchanges {
  Binance = 0,
}
#[derive(
  Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
)]
#[repr(i32)]
#[derive(
  ::serde::Serialize,
  ::serde::Deserialize,
  ::num_derive::FromPrimitive,
  ::clap::Clap,
)]
pub enum BackTestPriceBase {
  Close = 0,
  Open = 1,
  High = 2,
  Low = 3,
  OpenCloseMid = 4,
  HighLowMid = 5,
}
