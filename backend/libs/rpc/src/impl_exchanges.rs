use std::str::FromStr;

use ::errors::UnknownExchangeError;
use ::warp::Filter;

use crate::exchanges::Exchanges;

impl Exchanges {
  pub fn as_str(&self) -> &str {
    match self {
      Exchanges::Binance => "Binance",
    }
  }

  pub fn by_param(
  ) -> impl Filter<Extract = (Exchanges,), Error = ::warp::Rejection>
       + Clone
       + Send
       + Sync
       + 'static {
    return ::warp::path::param::<String>()
      .and_then(|param: String| async move {
        let exchanges: Result<Exchanges, _> = param.parse();
        let exchanges = exchanges
          .map(|exchange| (exchange,))
          .map_err(|_| ::warp::reject::not_found());
        return exchanges;
      })
      .untuple_one();
  }
}

impl ToString for Exchanges {
  fn to_string(&self) -> String {
    return self.as_str().to_string();
  }
}

impl FromStr for Exchanges {
  type Err = UnknownExchangeError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Binance" => Ok(Exchanges::Binance),
      _ => Err(UnknownExchangeError::new(s.to_string())),
    }
  }
}
