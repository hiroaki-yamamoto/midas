use ::async_trait::async_trait;
use ::nats::asynk::Connection as Broker;

use ::types::{GenericResult};

use super::constants::REST_ENDPOINT;
use super::client::PubClient;
use super::entities::ListenKey;

use crate::entities::APIKey;
use crate::traits::UserStream as UserStreamTrait;

#[derive(Debug, Clone)]
pub struct UserStream {
  broker: Broker,
  listen_keys: Vec<String>,
}

impl UserStream {
  fn new(broker: Broker) -> Self {
    return Self { broker, listen_keys: vec![] };
  }
}

impl PubClient for UserStream {}

#[async_trait]
impl UserStreamTrait for UserStream {
  async fn authenticate(&self, api_key: &APIKey) -> GenericResult<()> {
    let client = self.get_client(api_key.pub_key.to_owned())?;
    let resp: ListenKey = client
      .post(format!("{}/api/v3/userDataStream", REST_ENDPOINT).as_str())
      .send()
      .await?
      .json()
      .await?;
    return Ok(());
  }
}
