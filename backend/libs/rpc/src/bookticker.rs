#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BookTicker {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(double, tag = "3")]
    pub bid_price: f64,
    #[prost(double, tag = "4")]
    pub bid_qty: f64,
    #[prost(double, tag = "5")]
    pub ask_price: f64,
    #[prost(double, tag = "6")]
    pub ask_qty: f64,
}
