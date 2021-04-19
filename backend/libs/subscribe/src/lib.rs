mod handler;
mod streams;
mod traits;

pub use self::handler::handle;
pub use self::streams::{to_stream, to_stream_msg, to_stream_raw};
pub use self::traits::PubSub;

#[macro_export]
macro_rules! pubsub {
  ($accessor: vis, $name: ident, $entity: ty, $sub_name: expr) => {
    #[derive(Debug, Clone)]
    $accessor struct $name {
      con: ::nats::Connection
    }

    impl $name {
      pub fn new(con: ::nats::Connection) -> Self {
        return Self { con };
      }
      ::types::stateful_setter!(con, ::nats::Connection);
    }

    impl ::subscribe::PubSub<$entity> for $name {
      fn get_broker(&self) -> &Broker {
        return &self.con;
      }
      fn get_subject(&self) -> &str {
        return $sub_name;
      }
    }
  }
}
