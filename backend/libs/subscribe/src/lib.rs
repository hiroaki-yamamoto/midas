use ::std::io::{Error as IOError, ErrorKind};

use ::futures_core::stream::Stream;
use ::nats::subscription::{Handler, Subscription};
use ::tokio::sync::mpsc::unbounded_channel;
use ::tokio_stream::wrappers::UnboundedReceiverStream;

pub fn to_stream(sub: Subscription) -> (Handler, impl Stream) {
  let (sender, receiver) = unbounded_channel();
  let handler = sub.with_handler(move |msg| {
    return sender
      .send(msg)
      .map_err(|e| IOError::new(ErrorKind::Other, e));
  });
  let stream = UnboundedReceiverStream::new(receiver);
  return (handler, stream);
}
