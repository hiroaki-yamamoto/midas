#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsertOneResult {
    #[prost(string, tag="1")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: ::prost::alloc::string::String,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(::num_derive::FromPrimitive, ::clap::Clap)]
#[serde(tag = "exchange")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Exchanges {
    Binance = 0,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(::num_derive::FromPrimitive, ::clap::Clap)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BackTestPriceBase {
    Close = 0,
    Open = 1,
    High = 2,
    Low = 3,
    OpenCloseMid = 4,
    HighLowMid = 5,
}
