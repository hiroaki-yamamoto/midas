mod traits;

pub use ::log;
pub use ::tokio::time::sleep;

pub use self::traits::PubSub;
pub use ::async_nats as nats;
pub use ::async_nats::jetstream as natsJS;

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
      stream: ::async_nats::jetstream::stream::Stream,
    }

    impl $name {
      async fn add_stream(
        &self, ctx: async_nats::jetstream::context::Context,
      ) -> ::std::io::Result<::async_nats::jetstream::stream::Stream> {
        let mut option: ::async_nats::jetstream::stream::Config = $id.into();
        option.max_consumers = -1;
        return ctx.get_or_create_stream(option).await?;
      }

      pub async fn new(
        ctx: ::async_nats::jetstream::context::Context,
      ) -> ::std::io::Result<Self> {
        let stream = me.add_stream(ctx).await?;
        let mut me = Self { stream };
        return Ok(me);
      }
    }

    impl ::subscribe::PubSub<$entity> for $name {
      fn get_stream(&self) -> &::async_nats::jetstream::context::Context {
        return &self.stream;
      }
      fn get_subject(&self) -> &str {
        return $id;
      }
    }
  }
}
