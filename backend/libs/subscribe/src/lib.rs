mod handler;
mod streams;
mod traits;

pub use self::handler::handle;
pub use self::streams::{to_stream, to_stream_msg, to_stream_raw};
pub use self::traits::PubSub;

#[macro_export]
macro_rules! impl_pubsub {
  ($cls_name: ty, $type_name: ty, $sub_name: expr) => {
    impl $cls_name {
      pub fn new(con: ::nats::Connection) -> Self {
        return Self { con };
      }
      ::types::stateful_setter!(con, ::nats::Connection);
    }

    impl PubSub<$type_name> for $cls_name {
      fn get_broker(&self) -> &Broker {
        return &self.con;
      }
      fn get_subject(&self) -> &str {
        return $sub_name;
      }
    }
  };
}
