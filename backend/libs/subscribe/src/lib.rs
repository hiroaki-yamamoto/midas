mod traits;

pub use ::log;
pub use ::tokio::time::sleep;

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
      js: ::nats::jetstream::JetStream,
    }

    impl $name {
      async fn add_stream(&self) -> ::std::io::Result<()> {
        let mut option: ::nats::jetstream::StreamConfig = $id.into();
        option.max_consumers = -1;

        const MAX_RETRY: usize = 30;
        for count in 1..=MAX_RETRY {
          if let Err(e) = self.js.update_stream(&option) {
            ::subscribe::log::warn!(
              "Failed to update the stream. Retrying after
                1 sec...({}/{}): {}", count, e, MAX_RETRY
            );
          } else {
            match self.js.add_stream(&option) {
              Ok(_) => return Ok(()),
              Err(e) => {
                ::subscribe::log::warn!(
                  "Failed to acquire stream. Retrying after
                    1 sec...({}/{}): {}", count, e, MAX_RETRY
                );
              },
            }
          }
          ::subscribe::sleep(::std::time::Duration::from_secs(1)).await;
        }
        return Err(::std::io::Error::new(
          ::std::io::ErrorKind::Other,
          "Failed to acquire stream",
        ));
      }

      pub async fn new(js: ::nats::jetstream::JetStream) -> ::std::io::Result<Self> {
        let mut me = Self {js};
        me.add_stream().await?;
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
