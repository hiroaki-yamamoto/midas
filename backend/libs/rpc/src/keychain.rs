#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiKey {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::entities::Exchanges", tag = "2")]
    pub exchange: i32,
    #[prost(string, tag = "3")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub pub_key: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub prv_key: ::prost::alloc::string::String,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiKeyList {
    #[prost(message, repeated, tag = "1")]
    #[serde(rename = "keysList")]
    pub keys: ::prost::alloc::vec::Vec<ApiKey>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiRename {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
}
