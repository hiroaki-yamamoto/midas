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
    pub reinvest: bool,
    #[prost(string, tag="9")]
    pub condition: ::prost::alloc::string::String,
}
