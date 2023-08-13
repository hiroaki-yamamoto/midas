mod traits;

pub use ::log;
pub use ::tokio::time::interval;

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
      async fn add_stream(&self) -> ::std::io::Result<()> {
        let option: ::nats::jetstream::StreamConfig = $id.into();
        let mut sleep = ::subscribe::interval(::std::time::Duration::from_secs(1));
        for count in 1..=600 {
          if let Err(e) = self.js.update_stream(&option) {
            ::subscribe::log::warn!(
              "Failed to update the stream. Retrying after
                1 sec...({}/600): {}", count, e
            );
          } else {
            match self.js.add_stream(&option) {
              Ok(_) => return Ok(()),
              Err(e) => {
                ::subscribe::log::warn!(
                  "Failed to acquire stream. Retrying after
                    1 sec...({}/600): {}", count, e
                );
              },
            }
          }
          sleep.tick().await;
        }
        return Ok(());
      }

      fn add_consumer(&self) {
        let mut cfg: ::nats::jetstream::ConsumerConfig = $id.into();
        cfg.deliver_policy = ::nats::jetstream::DeliverPolicy::All;
        cfg.ack_policy = ::nats::jetstream::AckPolicy::Explicit;
        let _ = self.js.add_consumer($id, cfg);
      }

      pub async fn new(js: ::nats::jetstream::JetStream) -> ::std::io::Result<Self> {
        let me = Self { js };
        me.add_stream().await?;
        // me.add_consumer();
        return Ok(me);
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
