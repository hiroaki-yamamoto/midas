#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetIndicator {
    #[prost(oneof="target_indicator::Target", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16")]
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
        WatchPrice(super::super::google::protobuf::Empty),
        #[prost(message, tag="5")]
        CurrentVolume(super::super::google::protobuf::Duration),
        #[prost(message, tag="6")]
        VolumeLastTick(super::super::google::protobuf::Duration),
        #[prost(message, tag="7")]
        HighPriceLastTick(super::super::google::protobuf::Duration),
        #[prost(message, tag="8")]
        LowPriceLastTick(super::super::google::protobuf::Duration),
        #[prost(message, tag="9")]
        MidPriceLastTick(super::super::google::protobuf::Duration),
        #[prost(message, tag="10")]
        OpenPriceLastTick(super::super::google::protobuf::Duration),
        #[prost(message, tag="11")]
        ClosePriceLastTick(super::super::google::protobuf::Duration),
        #[prost(message, tag="12")]
        Sma(super::super::google::protobuf::Duration),
        #[prost(message, tag="13")]
        Ema(super::super::google::protobuf::Duration),
        #[prost(message, tag="14")]
        Rsi(super::super::google::protobuf::Duration),
        #[prost(message, tag="15")]
        Macd(super::super::google::protobuf::Duration),
        #[prost(message, tag="16")]
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
        And(super::Triggers),
        #[prost(message, tag="2")]
        Or(super::Triggers),
        #[prost(message, tag="3")]
        Not(::prost::alloc::boxed::Box<super::Trigger>),
        #[prost(message, tag="4")]
        Single(super::ConditionItem),
    }
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Triggers {
    #[prost(message, repeated, tag="1")]
    pub triggers: ::prost::alloc::vec::Vec<Trigger>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trailing {
    #[prost(message, optional, tag="1")]
    pub watch_point: ::core::option::Option<Trigger>,
    #[prost(message, optional, tag="2")]
    pub unwatch_point: ::core::option::Option<Trigger>,
    #[prost(message, optional, tag="3")]
    pub trigger_point: ::core::option::Option<Trigger>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Manual {
    #[prost(message, optional, tag="1")]
    pub entry_point: ::core::option::Option<Trailing>,
    #[prost(message, optional, tag="2")]
    pub exit_point: ::core::option::Option<Trailing>,
    #[prost(message, optional, tag="3")]
    pub loss_cut_point: ::core::option::Option<Trigger>,
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
        Manual(super::Manual),
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
    #[prost(string, tag="3")]
    pub base_currency: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub created_at: ::core::option::Option<super::google::protobuf::Timestamp>,
    #[prost(double, tag="5")]
    pub trading_amount: f64,
    #[prost(double, tag="6")]
    pub current_valuation: f64,
    #[prost(double, tag="7")]
    pub realized_profit: f64,
    #[prost(bool, tag="8")]
    pub auto_reinvestment: bool,
    #[prost(message, optional, tag="9")]
    pub trigger: ::core::option::Option<TriggerType>,
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
pub enum CompareOp {
    Eq = 0,
    Gt = 1,
    Gte = 2,
    Lt = 3,
    Lte = 4,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PositionStatus {
    Closed = 0,
    Opened = 1,
}
