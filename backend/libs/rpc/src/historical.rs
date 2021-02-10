#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistChartProg {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub num_symbols: u64,
    #[prost(uint64, tag="3")]
    pub cur_symbol_num: u64,
    #[prost(uint64, tag="4")]
    pub num_objects: u64,
    #[prost(uint64, tag="5")]
    pub cur_object_num: u64,
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
