#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bot {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub base_currency: ::prost::alloc::string::String,
    #[prost(enumeration="super::entities::Exchanges", tag="4")]
    pub exchange: i32,
    #[prost(message, optional, tag="5")]
    pub created_at: ::core::option::Option<super::google::protobuf::Timestamp>,
    #[prost(double, tag="6")]
    pub trade_amount: f64,
    #[prost(bool, tag="7")]
    pub reinvest: bool,
    #[prost(string, tag="8")]
    pub condition: ::prost::alloc::string::String,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub bot_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(enumeration="PositionStatus", tag="4")]
    pub status: i32,
    #[prost(double, tag="5")]
    pub trading_amount: f64,
    #[prost(double, tag="6")]
    pub valuation: f64,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TriggerType {
    Manual = 0,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PositionStatus {
    Closed = 0,
    Opened = 1,
}
