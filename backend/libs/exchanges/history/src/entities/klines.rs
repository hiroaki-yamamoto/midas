use crate::binance::entities::Kline as BinanceKline;

pub enum KlinesByExchange {
  Binance(Vec<BinanceKline>),
}
