use super::exchange::Exchange;
use super::{
  api_key::ExchangeEntity as APIKeyExchange,
  api_key_list::ExchangeEntity as APIKeyListExchange,
  bot::ExchangeEntity as BotExchange,
  history_fetch_request::ExchangeEntity as HistoryFetchReqExchange,
  history_progress::ExchangeEntity as HistoryProgExchange,
  status_check_request::ExchangeEntity as StatusReqExchange,
  symbol_info::ExchangeEntity as SymbolInfoExchange,
  symbol_list::ExchangeEntity as SymbolListExchange,
};

macro_rules! impl_cast {
    ($($name:ident),*) => {
        $(
            impl From<$name> for Exchange {
                fn from(v: $name) -> Self {
                    return match v {
                        $name::Binance => Self::Binance,
                    };
                }
            }

            impl From<Exchange> for $name {
                fn from(value: Exchange) -> Self {
                    return match value {
                        Exchange::Binance => Self::Binance,
                    };
                }
            }
        )*
    };
}

impl_cast!(
  APIKeyExchange,
  APIKeyListExchange,
  BotExchange,
  HistoryFetchReqExchange,
  HistoryProgExchange,
  StatusReqExchange,
  SymbolInfoExchange,
  SymbolListExchange
);
