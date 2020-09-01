use ::tonic::{async_trait, Request, Response};

use ::rpc::symbol::{symbol_server::Symbol, RefreshRequest};
use ::types::Result;

struct Service;

#[async_trait]
impl Symbol for Service {
  async fn refresh(
    &self,
    request: Request<RefreshRequest>,
  ) -> Result<Response<()>> {
    return Ok(Response::new(()));
  }
}
