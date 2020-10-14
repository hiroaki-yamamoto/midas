#[derive(Clone, PartialEq, ::prost::Message)]
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
  ::num_derive::FromPrimitive,
  ::serde::Serialize,
  ::serde::Deserialize,
  ::clap::Clap,
)]
#[serde(tag = "exchange")]
pub enum Exchanges {
  Binance = 0,
}
