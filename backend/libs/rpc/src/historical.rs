#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistChartProg {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub num_symbols: i64,
    #[prost(int64, tag="4")]
    pub cur_symbol_num: i64,
    #[prost(int64, tag="5")]
    pub num_objects: i64,
    #[prost(int64, tag="6")]
    pub cur_object_num: i64,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistChartFetchReq {
    #[prost(string, repeated, tag="2")]
    #[serde(rename = "symbolsList")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
    #[prost(enumeration="super::entities::Exchanges", repeated, tag="1")]
    pub exchanges: ::prost::alloc::vec::Vec<i32>,
}
