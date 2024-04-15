use ::std::sync::Arc;

use ::warp::filters::BoxedFilter;
use ::warp::{Filter, Reply};

use ::rpc::exchanges::Exchanges;
use ::rpc::position_query::PositionQuery;

use crate::context::Context;

pub fn construct(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  return ::warp::get()
    .and(Exchanges::by_param())
    .and(::warp::query())
    .and_then(|exchange: Exchanges, qs: PositionQuery| async move {
      return Ok((ctx.clone(), exchange, qs));
    })
    .untuple_one()
    .map(|ctx: Arc<Context>, exchange: Exchanges, qs: PositionQuery| {})
    .boxed();
}
