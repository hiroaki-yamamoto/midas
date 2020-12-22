#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    #[prost(string, tag="1")]
    pub id: std::string::String,
    #[prost(enumeration="super::entities::Exchanges", tag="2")]
    pub exchange: i32,
    #[prost(string, tag="3")]
    pub label: std::string::String,
    #[prost(string, tag="4")]
    pub pub_key: std::string::String,
    #[prost(string, tag="5")]
    pub prv_key: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyList {
    #[prost(message, repeated, tag="1")]
    pub keys: ::std::vec::Vec<ApiKey>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiRename {
    #[prost(string, tag="1")]
    pub label: std::string::String,
}
