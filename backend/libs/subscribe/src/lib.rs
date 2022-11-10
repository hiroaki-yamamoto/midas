mod traits;

pub use self::traits::PubSub;

#[macro_export]
macro_rules! pubsub {
  (
    $accessor: vis,
    $name: ident,
    $entity: ty,
    $id: expr,
  ) => {
    #[derive(Debug, Clone)]
    $accessor struct $name {
      js: ::nats::jetstream::JetStream
    }

    impl $name {
      fn add_stream(&self){
        let mut option: ::nats::jetstream::StreamConfig = $id.into();
        option.retention = ::nats::jetstream::RetentionPolicy::WorkQueue;
        let _ = self.js.update_stream(&option);
        let _ = self.js.add_stream(option);
      }

      fn add_consumer(&self) {
        let mut cfg: ::nats::jetstream::ConsumerConfig = $id.into();
        cfg.deliver_policy = ::nats::jetstream::DeliverPolicy::All;
        cfg.ack_policy = ::nats::jetstream::AckPolicy::Explicit;
        let _ = self.js.add_consumer($id, cfg);
      }

      pub fn new(js: ::nats::jetstream::JetStream) -> Self {
        let me = Self { js };
        me.add_stream();
        // me.add_consumer();
        return me;
      }
    }

    impl ::subscribe::PubSub<$entity> for $name {
      fn get_natsjs(&self) -> &::nats::jetstream::JetStream {
        return &self.js;
      }
      fn get_subject(&self) -> &str {
        return $id;
      }
    }
  }
}
