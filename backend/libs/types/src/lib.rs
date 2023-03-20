pub mod casting;
pub mod stateful_setter;

use ::tokio::net::TcpStream;
use ::tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub type TLSWebSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;
