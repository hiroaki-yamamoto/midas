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
pub struct Condition {
    #[prost(oneof="condition::Condition", tags="1, 2, 3, 4")]
    pub condition: ::core::option::Option<condition::Condition>,
}
/// Nested message and enum types in `Condition`.
pub mod condition {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Condition {
        #[prost(message, tag="1")]
        And(::prost::alloc::boxed::Box<super::Condition>),
        #[prost(message, tag="2")]
        Or(::prost::alloc::boxed::Box<super::Condition>),
        #[prost(message, tag="3")]
        Not(::prost::alloc::boxed::Box<super::Condition>),
        #[prost(message, tag="4")]
        Single(super::ConditionItem),
    }
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
