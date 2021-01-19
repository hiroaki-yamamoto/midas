use ::tokio::net::TcpStream;
use ::tokio_native_tls::TlsStream;
use ::tokio_tungstenite::{stream::Stream, WebSocketStream};

pub type TLSWebSocket =
  WebSocketStream<Stream<TcpStream, TlsStream<TcpStream>>>;
