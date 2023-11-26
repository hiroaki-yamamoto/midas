pub mod casting;
pub mod stateful_setter;

pub use ::chrono;
use ::tokio::net::TcpStream;
use ::tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub type TLSWebSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub type DateTime = ::chrono::DateTime<::chrono::Utc>;
