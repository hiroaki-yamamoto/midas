#[derive(
  Clone, PartialEq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest {
  #[prost(enumeration = "super::entities::Exchanges", tag = "1")]
  pub exchange: i32,
}
#[derive(
  Clone, PartialEq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct QueryRequest {
  #[prost(enumeration = "super::entities::Exchanges", tag = "1")]
  pub exchange: i32,
  #[prost(string, tag = "2")]
  pub status: std::string::String,
  #[prost(string, repeated, tag = "3")]
  pub symbols: ::std::vec::Vec<std::string::String>,
}
#[derive(
  Clone, PartialEq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct QueryResponse {
  #[prost(message, repeated, tag = "1")]
  pub symbols: ::std::vec::Vec<super::entities::SymbolInfo>,
}
