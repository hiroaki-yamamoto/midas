mod traits;

pub use ::log;
pub use ::tokio::time::sleep;

pub use self::traits::PubSub;
pub use ::async_nats as nats;
pub use ::async_nats::jetstream as natsJS;
pub use ::bytes;
pub use ::errors;

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
      stream: ::subscribe::natsJS::stream::Stream,
      ctx: ::subscribe::natsJS::context::Context,
      cli: ::subscribe::nats::client::Client
    }

    impl $name {
      async fn add_stream(
        ctx: &::subscribe::natsJS::context::Context,
      ) -> ::subscribe::errors::CreateStreamResult<::subscribe::natsJS::stream::Stream> {
        let mut option: ::subscribe::natsJS::stream::Config = $id.into();
        option.max_consumers = -1;
        return ctx.get_or_create_stream(option).await;
      }

      pub async fn new(
        cli: &::subscribe::nats::client::Client,
      ) -> ::subscribe::errors::CreateStreamResult<Self> {
        let ctx = ::subscribe::natsJS::new(cli.clone());
        let stream = Self::add_stream(&ctx).await?;
        let mut me = Self { stream, ctx, cli: cli.clone() };
        return Ok(me);
      }
    }

    impl ::subscribe::PubSub<$entity> for $name {
      fn get_ctx(&self) -> &::subscribe::natsJS::context::Context {
        return &self.ctx;
      }
      fn get_stream(&self) -> &::subscribe::natsJS::stream::Stream {
        return &self.stream;
      }
      fn get_subject(&self) -> &str {
        return $id;
      }
    }
  }
}
