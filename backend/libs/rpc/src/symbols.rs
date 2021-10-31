#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolInfo {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub quote: ::prost::alloc::string::String,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseCurrencyList {
    #[prost(string, repeated, tag="1")]
    pub base_currencies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
