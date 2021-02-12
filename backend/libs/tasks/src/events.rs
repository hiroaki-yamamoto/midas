use ::futures::future::Future;
use ::futures::{Stream, StreamExt};
use ::tokio::select;
use ::tokio::sync::broadcast::Receiver;

pub async fn handle_async_stream<F, T>(
  mut stream: impl Stream<Item = T> + Send + Unpin + 'static,
  mut stop: Receiver<()>,
  handle_fn: impl Fn(T) -> F + Send + Sync + 'static,
) -> ::tokio::task::JoinHandle<F::Output>
where
  F: Future<Output = ()> + Send,
  T: Send,
{
  return tokio::spawn(async move {
    loop {
      select! {
        Some(msg) = stream.next() => {
          handle_fn(msg).await;
        },
        _ = stop.recv() => {
          break;
        },
      }
    }
  });
}
