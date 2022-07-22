mod streams;
mod traits;

pub use self::traits::PubSub;

#[macro_export]
macro_rules! pubsub {
  ($accessor: vis, $name: ident, $entity: ty, $sub_name: expr) => {
    #[derive(Debug, Clone)]
    $accessor struct $name {
      js: ::nats::jetstream::JetStream
    }

    impl $name {
      pub fn new(js: ::nats::jetstream::JetStream) -> Self {
        return Self { js };
      }
      ::types::stateful_setter!(js, ::nats::jetstream::JetStream);
    }

    impl ::subscribe::PubSub<$entity> for $name {
      fn get_natsjs(&self) -> &::nats::jetstream::JetStream {
        return &self.js;
      }
      fn get_subject(&self) -> &str {
        return $sub_name;
      }
    }
  }
}
