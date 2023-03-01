pub mod casting;
pub mod stateful_setter;

use ::std::error::Error;
use ::std::result::Result as StdResult;

use ::tokio::net::TcpStream;
use ::tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use ::url::{ParseError, Url};

pub type ParseURLResult = StdResult<Url, ParseError>;
pub type GenericResult<T> = StdResult<T, Box<dyn Error>>;
pub type ThreadSafeResult<T> = StdResult<T, Box<dyn Error + Send + Sync>>;
pub type TLSWebSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;
