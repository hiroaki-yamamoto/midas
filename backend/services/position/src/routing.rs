use ::std::str::FromStr;
use ::std::sync::Arc;

use ::futures::TryFutureExt;
use ::mongodb::bson::oid::ObjectId;
use ::warp::filters::BoxedFilter;
use ::warp::{Filter, Reply};

use ::rpc::position_query::PositionQuery;
use ::rpc::status::{Status, StatusCode};

use crate::context::Context;

pub fn construct(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  return list_by_bot(&ctx);
}

fn list_by_bot(ctx: &Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  return ::warp::get()
    .and(::warp::path("by-bot"))
    .and(::warp::path::param())
    .and(::warp::query())
    .and_then(|bot_id: String, qs: PositionQuery| async move {
      let bot_id = ObjectId::from_str(&bot_id).map_err(|e| {
        let code = StatusCode::EXPECTATION_FAILED;
        let status = Status::new(code.clone(), &e.to_string());
        return ::warp::reject::custom(status);
      })?;
      return Ok((ctx.clone(), bot_id, qs));
    })
    .untuple_one()
    .and_then(
      |ctx: Arc<Context>, bot_id: ObjectId, qs: PositionQuery| async move {
        let positions = ctx
          .position_repo
          .list_by_bot_id(bot_id, qs.pagination.clone())
          .map_err(|e| {
            let code = if StatusCode::INTERNAL_SERVER_ERROR;
            let status = Status::new(code.clone(), &e.to_string());
            return ::warp::reject::custom(status);
          })
          .await?;
      },
    )
    .boxed();
}
