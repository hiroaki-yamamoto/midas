use ::std::ops::Drop;
use ::std::task::Poll;

use ::async_trait::async_trait;
use ::errors::{ObserverError, ObserverResult, ParseResult};
use ::futures::{SinkExt, Stream, StreamExt};
use ::log::{as_error, as_serde, debug, info};
use ::random::generate_random_txt;
use ::rug::Float;

use ::clients::binance::WS_ENDPOINT;
use ::round_robin_client::{entities::WSMessageDetail, WebSocket};

use crate::binance::entities::{
  BookTicker, SubscribeRequest, SubscribeRequestInner, WebsocketPayload,
};

use super::interfaces::{BookTickerStream, IBookTickerSocket};

pub struct BookTickerSocket {
  socket: WebSocket<WebsocketPayload, SubscribeRequest>,
  symbols: Vec<String>,
  id: String,
}

impl BookTickerSocket {
  pub async fn new() -> ObserverResult<Self> {
    let socket = WebSocket::new(WS_ENDPOINT).await?;
    let inst = Self {
      socket,
      symbols: Vec::new(),
      id: generate_random_txt(36),
    };
    return Ok(inst);
  }

  #[cfg(test)]
  pub(crate) async fn test_new(url: &str) -> ObserverResult<Self> {
    let socket = WebSocket::new(&[url]).await?;
    let inst = Self {
      socket,
      symbols: Vec::new(),
      id: generate_random_txt(36),
    };
    return Ok(inst);
  }

  fn parse_payload_inner(
    &mut self,
    payload: WebsocketPayload,
  ) -> WSMessageDetail<BookTicker<Float>> {
    match payload {
      WebsocketPayload::BookTicker(book_ticker) => {
        info!(payload = as_serde!(book_ticker); "Received Payload");
        let book_ticker: ParseResult<BookTicker<Float>> =
          book_ticker.try_into();
        return match book_ticker {
          Ok(book_ticker) => WSMessageDetail::EntityReceived(book_ticker),
          Err(error) => {
            info!(error=as_error!(error); "Error");
            WSMessageDetail::Continue
          }
        };
      }
      WebsocketPayload::Error(error) => {
        info!(error=as_serde!(&error); "Error");
        WSMessageDetail::Continue
      }
      WebsocketPayload::Result(result) => {
        info!(result=as_serde!(&result); "Unexpected Result");
        WSMessageDetail::Continue
      }
    }
  }

  fn parse_payload(
    &mut self,
    payload: WSMessageDetail<WebsocketPayload>,
  ) -> Option<WSMessageDetail<BookTicker<Float>>> {
    return match payload {
      WSMessageDetail::Continue => Some(WSMessageDetail::Continue),
      WSMessageDetail::Disconnected => Some(WSMessageDetail::Disconnected),
      WSMessageDetail::EntityReceived(payload) => {
        let parsed = self.parse_payload_inner(payload);
        Some(parsed)
      }
    };
  }
}

#[async_trait]
impl IBookTickerSocket for BookTickerSocket {
  fn symbols(&self) -> &[String] {
    return &self.symbols;
  }

  fn has_symbol(&self, symbol: &str) -> bool {
    return self.symbols.contains(&symbol.to_string());
  }

  fn len(&self) -> usize {
    return self.symbols.len();
  }

  async fn subscribe(&mut self, symbols: &[String]) -> ObserverResult<()> {
    let payload = SubscribeRequestInner {
      id: self.id.clone(),
      params: symbols
        .iter()
        .map(|symbol| format!("{}@bookTicker", symbol.to_lowercase()))
        .collect(),
    }
    .into_subscribe();
    debug!(payload = as_serde!(payload); "Start Subscribe");
    self.socket.send(payload).await?;
    let result = self.socket.next().await;
    if let Some(WSMessageDetail::EntityReceived(WebsocketPayload::Result(
      result,
    ))) = &result
    {
      if result.id == self.id {
        info!(result=as_serde!(&result); "Subscribed.");
        return Ok(());
      }
    }
    self.symbols.append(&mut symbols.to_vec());
    debug!(result = as_serde!(result); "Failed to subscribe.");
    return Err(ObserverError::Other("Failed to subscribe.".to_string()));
  }

  async fn unsubscribe(&mut self, symbols: &[String]) -> ObserverResult<()> {
    let payloads: Vec<SubscribeRequest> = symbols
      .iter()
      .filter(|symbol| self.has_symbol(symbol))
      .map(|symbol| {
        SubscribeRequestInner {
          id: self.id.clone(),
          params: vec![format!("{}@bookTicker", symbol.to_lowercase())],
        }
        .into_unsubscribe()
      })
      .collect();
    for payload in payloads {
      debug!(payload = as_serde!(payload); "Start Unsubscribe");
      self.socket.send(payload).await?;
    }
    // Remove symbols from the map
    self.symbols.retain(|symbol| !symbols.contains(symbol));

    return Ok(());
  }
}

impl From<BookTickerSocket> for BookTickerStream {
  fn from(socket: BookTickerSocket) -> Self {
    return Box::pin(socket);
  }
}

impl Stream for BookTickerSocket {
  type Item = WSMessageDetail<BookTicker<Float>>;

  fn poll_next(
    mut self: ::std::pin::Pin<&mut Self>,
    cx: &mut ::std::task::Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    if let Poll::Ready(payload) = self.socket.poll_next_unpin(cx) {
      return match payload {
        None => Poll::Ready(Some(WSMessageDetail::Disconnected)),
        Some(payload) => {
          let book_ticker = self.parse_payload(payload);
          Poll::Ready(book_ticker)
        }
      };
    }
    return Poll::Pending;
  }
}

impl Drop for BookTickerSocket {
  fn drop(&mut self) {
    let _ = self.socket.close();
  }
}
