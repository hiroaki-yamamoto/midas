use ::async_trait::async_trait;

use ::round_robin_client::RestClient;

use super::super::interfaces::IListenKeyClient;

pub struct ListenKeyClient {
  cli: RestClient,
}

impl ListenKeyClient {
  pub fn new() -> Self {
    let cli = RestClient::new();
    Self { cli }
  }
}
