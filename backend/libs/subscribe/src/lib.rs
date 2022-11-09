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
      fn add_stream(&self) -> std::io::Result<Option<::nats::jetstream::StreamInfo>> {
        let con = &self.js;
        let stream_name = $sub_name;
        if con
          .list_streams()
          .filter_map(|st_res| st_res.ok())
          .find(|st_info| st_info.config.name == stream_name)
          .is_none()
        {
          return Ok(Some(con.add_stream(stream_name)?));
        }
        return Ok(None);
      }
      pub fn new(js: ::nats::jetstream::JetStream) -> Self {
        let me = Self { js };
        let _ = me.add_stream();
        return me;
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
