#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolInfo {
    #[prost(enumeration = "super::entities::Exchanges", tag = "1")]
    pub exchange: i32,
    #[prost(enumeration = "Type", tag = "2")]
    pub r#type: i32,
    #[prost(string, tag = "3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub quote: ::prost::alloc::string::String,
    #[prost(int64, tag = "7")]
    pub base_precision: i64,
    #[prost(int64, tag = "8")]
    pub quote_precision: i64,
    #[prost(int64, tag = "9")]
    pub base_commission_precision: i64,
    #[prost(int64, tag = "10")]
    pub quote_commission_precision: i64,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymbolList {
    #[prost(message, repeated, tag = "1")]
    pub symbols: ::prost::alloc::vec::Vec<SymbolInfo>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseSymbols {
    #[prost(string, repeated, tag = "1")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Type {
    Crypto = 0,
    Stock = 1,
}
impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Type::Crypto => "CRYPTO",
            Type::Stock => "STOCK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CRYPTO" => Some(Self::Crypto),
            "STOCK" => Some(Self::Stock),
            _ => None,
        }
    }
}
