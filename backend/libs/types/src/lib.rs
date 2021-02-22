pub mod casting;
pub mod errors;

use ::std::error::Error;
use ::std::result::Result as StdResult;

use ::chrono::{DateTime as ChronoDateTime, Utc};
use ::tokio::net::TcpStream;
use ::tokio_native_tls::TlsStream;
use ::tokio_tungstenite::{stream::Stream, WebSocketStream};
use ::url::{ParseError, Url};

pub type ParseURLResult = StdResult<Url, ParseError>;
pub type GenericResult<T> = StdResult<T, Box<dyn Error>>;
pub type ThreadSafeResult<T> = StdResult<T, Box<dyn Error + Send + Sync>>;
pub type DateTime = ChronoDateTime<Utc>;
pub type TLSWebSocket =
  WebSocketStream<Stream<TcpStream, TlsStream<TcpStream>>>;

#[macro_export]
macro_rules! reply_on_err {
  ($code: expr, $result: expr) => {
    match $result {
      Err(err) => {
        let resp: Box<dyn ::warp::Reply> =
          Box::new(::warp::reply::with_status(
            ::warp::reply::json(&::types::Status::new(
              $code,
              format!("{}", err).as_str(),
            )),
            $code,
          ));
        return resp;
      }
      Ok(v) => v,
    }
  };
}
