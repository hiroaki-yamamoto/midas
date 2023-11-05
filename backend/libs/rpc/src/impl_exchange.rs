use ::errors::ParseError;
use ::std::string::ToString;
use ::warp::{Filter, Rejection};

use super::exchange::Exchange;

impl Exchange {
  pub fn by_param() -> impl Filter<Extract = (Exchange,), Error = Rejection>
       + Clone
       + Send
       + Sync
       + 'static {
    return ::warp::path::param::<String>()
      .and_then(|param: String| async move {
        let exchange: Exchange = param
          .to_lowercase()
          .parse()
          .map_err(|_| ::warp::reject::not_found())?;
        return Ok::<(Exchange,), Rejection>((exchange,));
      })
      .untuple_one();
  }
}

impl ToString for Exchange {
  fn to_string(&self) -> String {
    return match self {
      &Self::Binance => "Binance".to_string(),
    };
  }
}

impl ::std::str::FromStr for Exchange {
  type Err = ParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let ret: Self = match s.to_lowercase().as_str() {
      "binance" => Exchange::Binance,
      _ => return Err(ParseError::new(None::<&str>, Some(s), None::<&str>)),
    };
    return Ok(ret);
  }
}
