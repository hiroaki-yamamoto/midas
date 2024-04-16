use ::std::str::FromStr;
use ::std::sync::Arc;

use ::futures::{StreamExt, TryFutureExt};
use ::log::warn;
use ::mongodb::bson::oid::ObjectId;
use ::warp::filters::BoxedFilter;
use ::warp::reply::json;
use ::warp::{Filter, Rejection, Reply};

use ::rpc::position::Position as PositionRpc;
use ::rpc::position_list::PositionList;
use ::rpc::position_query::PositionQuery;
use ::rpc::status::{Status, StatusCode};

use crate::context::Context;

pub fn construct(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  return list_by_bot(ctx.clone());
}

fn list_by_bot(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  return ::warp::get()
    .and(::warp::path("by-bot"))
    .and(::warp::path::param())
    .and(::warp::query())
    .map(move |bot_id: String, qs: PositionQuery| {
      return (ctx.clone(), bot_id, qs);
    })
    .untuple_one()
    .and_then(
      |ctx: Arc<Context>, bot_id: String, qs: PositionQuery| async move {
        let bot_id = ObjectId::from_str(&bot_id).map_err(|e| {
          let code = StatusCode::EXPECTATION_FAILED;
          let status = Status::new(code.clone(), &e.to_string());
          return ::warp::reject::custom(status);
        })?;
        return Ok::<_, Rejection>((ctx, bot_id, qs));
      },
    )
    .untuple_one()
    .and_then(
      |ctx: Arc<Context>, bot_id: ObjectId, qs: PositionQuery| async move {
        let conv = ctx.position_converter.clone();
        #[cfg(debug_assertions)]
        let pos_repo = if qs.demo_mode {
          ctx.position_demo_repo.clone()
        } else {
          ctx.position_repo.clone()
        };
        #[cfg(not(debug_assertions))]
        let pos_repo = ctx.position_repo.clone();

        let positions: Vec<Box<PositionRpc>> = pos_repo
          .list_by_bot_id(bot_id, *qs.pagination.clone())
          .map_err(|e| {
            let code = StatusCode::INTERNAL_SERVER_ERROR;
            let status = Status::new(code.clone(), &e.to_string());
            return ::warp::reject::custom(status);
          })
          .await?
          .filter_map(|pos_res| async move {
            if let Err(ref e) = pos_res {
              warn!(error: err = e.clone(); "Failed to fetch item");
            }
            return pos_res.ok();
          })
          .map(move |pos| (pos, conv.clone()))
          .filter_map(|(pos, conv)| async move {
            let rpc = conv.to_rpc(&pos).await;
            if let Err(ref e) = rpc {
              warn!(error: err = e.clone(); "Failed to encode item for frontend");
            }
            return rpc.ok();
          })
          .map(|pos| Box::new(pos))
          .collect()
          .await;
        return Ok::<_, Rejection>(PositionList { positions });
      },
    )
    .map(|list: PositionList| {
      return json(&list);
    })
    .boxed();
}
