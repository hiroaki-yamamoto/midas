use ::std::convert::TryFrom;

use ::errors::UnknownExchangeError;
use ::rpc::exchange::Exchange;

use crate::binance::entities::Kline as BinanceKline;

pub enum KlinesByExchange {
  Binance(Vec<BinanceKline>),
}

impl TryFrom<KlinesByExchange> for Vec<BinanceKline> {
  type Error = UnknownExchangeError;
  fn try_from(value: KlinesByExchange) -> Result<Self, Self::Error> {
    return match value {
      KlinesByExchange::Binance(klines) => Ok(klines),
    };
  }
}

impl KlinesByExchange {
  pub fn exchange(&self) -> Exchange {
    return match self {
      &Self::Binance(_) => Exchange::Binance,
    };
  }
}
