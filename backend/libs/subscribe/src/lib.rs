pub mod traits;

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
      ctx: ::subscribe::natsJS::context::Context,
      cli: ::subscribe::nats::client::Client,
    }

    impl $name {
      pub async fn new(
        cli: &::subscribe::nats::client::Client,
      ) -> ::subscribe::errors::CreateStreamResult<Self> {
        let ctx = ::subscribe::natsJS::new(cli.clone());
        let mut me = Self { ctx, cli: cli.clone() };
        return Ok(me);
      }
    }

    impl ::subscribe::PubSub for $name {
      type Output = $entity;
      fn get_client(&self) -> &::subscribe::nats::client::Client {
        return &self.cli;
      }
      fn get_ctx(&self) -> &::subscribe::natsJS::context::Context {
        return &self.ctx;
      }
      fn get_subject(&self) -> &str {
        return $id;
      }
    }
  }
}
