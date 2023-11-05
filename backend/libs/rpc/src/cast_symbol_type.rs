use super::symbol_type::SymbolType;
use super::{
  symbol_info::SymbolTypeSchema as SymbolInfoType,
  symbol_list::SymbolTypeSchema as SymbolListType,
};

macro_rules! impl_cast {
    ($($name:ident),*) => {
        $(
            impl From<$name> for SymbolType {
                fn from(v: $name) -> Self {
                    return match v {
                        $name::Crypto => Self::Crypto,
                        $name::Stock => Self::Stock,
                    };
                }
            }

            impl From<SymbolType> for $name {
                fn from(value: SymbolType) -> Self {
                    return match value {
                        SymbolType::Crypto => Self::Crypto,
                        SymbolType::Stock => Self::Stock,
                    };
                }
            }
        )*
    };
}

impl_cast!(SymbolListType, SymbolInfoType);
