use ::serde::Serialize;

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum WSMessageDetail<T> {
  /// The entity is actually received.
  EntityReceived(T),
  /// The message was a ping/pong message that has already proceeded.
  /// i.e. Pong will be sent, and pong will be ignored.
  Continue,
  /// The socket seems to be disconnected.
  Disconnected,
}
