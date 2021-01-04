#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BotInfo {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration="Strategy", tag="2")]
    pub strategy: i32,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub base_currency: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub desc: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub config: ::prost::alloc::string::String,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentPosition {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub bot_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(double, tag="4")]
    pub trading_amount: f64,
    #[prost(double, tag="5")]
    pub profit_amount: f64,
    #[prost(double, tag="6")]
    pub profit_percent: f64,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BotInfoList {
    #[prost(message, repeated, tag="1")]
    pub bots: ::prost::alloc::vec::Vec<BotInfo>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BotInfoListRequest {
    #[prost(int64, tag="1")]
    pub offset: i64,
    #[prost(int64, tag="2")]
    pub limit: i64,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Strategy {
    Trailing = 0,
}
