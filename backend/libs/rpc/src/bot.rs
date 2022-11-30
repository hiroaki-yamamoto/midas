#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bot {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub created_at: ::core::option::Option<super::google::protobuf::Timestamp>,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "super::entities::Exchanges", tag = "4")]
    pub exchange: i32,
    #[prost(string, tag = "5")]
    pub base_currency: ::prost::alloc::string::String,
    #[prost(double, tag = "6")]
    pub trading_amount: f64,
    #[prost(string, tag = "7")]
    pub condition: ::prost::alloc::string::String,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub bot_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(enumeration = "PositionStatus", tag = "4")]
    pub status: i32,
    #[prost(double, tag = "5")]
    pub trading_amount: f64,
    #[prost(double, tag = "6")]
    pub valuation: f64,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TriggerType {
    Manual = 0,
}
impl TriggerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TriggerType::Manual => "MANUAL",
        }
    }
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PositionStatus {
    Closed = 0,
    Opened = 1,
}
impl PositionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PositionStatus::Closed => "closed",
            PositionStatus::Opened => "opened",
        }
    }
}
