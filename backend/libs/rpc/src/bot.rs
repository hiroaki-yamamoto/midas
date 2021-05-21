#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetIndicator {
    #[prost(oneof="target_indicator::Target", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15")]
    pub target: ::core::option::Option<target_indicator::Target>,
}
/// Nested message and enum types in `TargetIndicator`.
pub mod target_indicator {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        #[prost(double, tag="1")]
        Absolute(f64),
        #[prost(double, tag="2")]
        Percentage(f64),
        #[prost(message, tag="3")]
        CurrentPrice(super::super::google::protobuf::Empty),
        #[prost(message, tag="4")]
        CurrentVolume(super::super::google::protobuf::Duration),
        #[prost(message, tag="5")]
        VolumeLastTick(super::super::google::protobuf::Duration),
        #[prost(message, tag="6")]
        HighPriceLastTick(super::super::google::protobuf::Duration),
        #[prost(message, tag="7")]
        LowPriceLastTick(super::super::google::protobuf::Duration),
        #[prost(message, tag="8")]
        MidPriceLastTick(super::super::google::protobuf::Duration),
        #[prost(message, tag="9")]
        OpenPriceLastTick(super::super::google::protobuf::Duration),
        #[prost(message, tag="10")]
        ClosePriceLastTick(super::super::google::protobuf::Duration),
        #[prost(message, tag="11")]
        Sma(super::super::google::protobuf::Duration),
        #[prost(message, tag="12")]
        Ema(super::super::google::protobuf::Duration),
        #[prost(message, tag="13")]
        Rsi(super::super::google::protobuf::Duration),
        #[prost(message, tag="14")]
        Macd(super::super::google::protobuf::Duration),
        #[prost(message, tag="15")]
        Cci(super::super::google::protobuf::Duration),
    }
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionItem {
    #[prost(enumeration="CompareOp", tag="1")]
    pub cmp: i32,
    #[prost(message, optional, tag="2")]
    pub op_a: ::core::option::Option<TargetIndicator>,
    #[prost(message, optional, tag="3")]
    pub op_b: ::core::option::Option<TargetIndicator>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trigger {
    #[prost(oneof="trigger::Trigger", tags="1, 2, 3, 4")]
    pub trigger: ::core::option::Option<trigger::Trigger>,
}
/// Nested message and enum types in `Trigger`.
pub mod trigger {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Trigger {
        #[prost(message, tag="1")]
        And(::prost::alloc::boxed::Box<super::Trigger>),
        #[prost(message, tag="2")]
        Or(::prost::alloc::boxed::Box<super::Trigger>),
        #[prost(message, tag="3")]
        Not(::prost::alloc::boxed::Box<super::Trigger>),
        #[prost(message, tag="4")]
        Single(super::ConditionItem),
    }
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerType {
    #[prost(oneof="trigger_type::Type", tags="1")]
    pub r#type: ::core::option::Option<trigger_type::Type>,
}
/// Nested message and enum types in `TriggerType`.
pub mod trigger_type {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="1")]
        Manual(super::Trigger),
    }
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bot {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub created_at: ::core::option::Option<super::google::protobuf::Timestamp>,
    #[prost(double, tag="4")]
    pub trading_amount: f64,
    #[prost(double, tag="5")]
    pub current_valuation: f64,
    #[prost(double, tag="6")]
    pub realized_profit: f64,
    #[prost(bool, tag="7")]
    pub auto_reinvestment: bool,
    #[prost(message, optional, tag="8")]
    pub trigger: ::core::option::Option<TriggerType>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CompareOp {
    Eq = 0,
    Gt = 1,
    Gte = 2,
    Lt = 3,
    Lte = 4,
}
