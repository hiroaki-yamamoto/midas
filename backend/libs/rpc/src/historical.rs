#[derive(
  Clone, PartialEq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct HistChartProg {
  #[prost(string, tag = "1")]
  pub symbol: std::string::String,
  #[prost(int64, tag = "2")]
  pub num_symbols: i64,
  #[prost(int64, tag = "3")]
  pub cur_symbol_num: i64,
  #[prost(int64, tag = "4")]
  pub num_objects: i64,
  #[prost(int64, tag = "5")]
  pub cur_object_num: i64,
}
#[derive(
  Clone, PartialEq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct HistChartFetchReq {
  #[prost(string, repeated, tag = "2")]
  pub symbols: ::std::vec::Vec<std::string::String>,
}
#[derive(
  Clone, PartialEq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct StopRequest {
  #[prost(enumeration = "super::entities::Exchanges", repeated, tag = "1")]
  pub exchanges: ::std::vec::Vec<i32>,
}
