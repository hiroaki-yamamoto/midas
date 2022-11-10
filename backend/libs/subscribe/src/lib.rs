mod traits;

pub use self::traits::PubSub;

#[macro_export]
macro_rules! pubsub {
  (
    $accessor: vis,
    $name: ident,
    $entity: ty,
    $stream_name: expr,
    $consumer_name: expr,
    $sub_name: expr,
  ) => {
    #[derive(Debug, Clone)]
    $accessor struct $name {
      js: ::nats::jetstream::JetStream
    }

    impl $name {
      fn add_stream(&self) -> std::io::Result<::nats::jetstream::StreamInfo> {
        let con = &self.js;
        let stream_name = $stream_name;
        let err = match self.js.stream_info(stream_name) {
          Ok(info) => return Ok(info),
          Err(e) => e,
        };
        if err.kind() != ::std::io::ErrorKind::Other {
          return Err(err);
        }
        if let Some(inner) = err.get_ref() {
          let inner: Option<&::nats::jetstream::Error> = inner.downcast_ref();
          if let Some(inner) = inner {
            if inner.error_code() == ::nats::jetstream::ErrorCode::StreamNotFound {
              let mut option: ::nats::jetstream::StreamConfig = stream_name.into();
              option.retention = ::nats::jetstream::RetentionPolicy::WorkQueue;
              return con.add_stream(option);
            }
          }
        }
        return Err(err);
      }
      fn add_consumer(&self) -> std::io::Result<::nats::jetstream::ConsumerInfo> {
        let err = match self.js.consumer_info($stream_name, $consumer_name) {
          Ok(info) => return Ok(info),
          Err(e) => e,
        };
        if err.kind() != ::std::io::ErrorKind::Other {
          return Err(err);
        }
        if let Some(inner) = err.get_ref() {
          let inner: Option<&::nats::jetstream::Error> = inner.downcast_ref();
          if let Some(inner) = inner {
            if inner.error_code() == ::nats::jetstream::ErrorCode::ConsumerNotFound {
              let mut cfg: ::nats::jetstream::ConsumerConfig = $consumer_name.into();
              cfg.deliver_policy = ::nats::jetstream::DeliverPolicy::All;
              cfg.ack_policy = ::nats::jetstream::AckPolicy::Explicit;
              return self.js.add_consumer($stream_name, cfg);
            }
          }
        }
        return Err(err)
      }
      pub fn new(js: ::nats::jetstream::JetStream) -> Self {
        let me = Self { js };
        let _ = me.add_stream();
        let _ = me.add_consumer();
        return me;
      }
    }

    impl ::subscribe::PubSub<$entity> for $name {
      fn get_natsjs(&self) -> &::nats::jetstream::JetStream {
        return &self.js;
      }
      fn get_subject(&self) -> &str {
        return $sub_name;
      }
    }

    impl ::std::ops::Drop for $name {
      fn drop(&mut self) {
        let _ = self.js.delete_consumer($stream_name, $consumer_name);
        let _ = self.js.delete_stream($stream_name);
      }
    }
  }
}
