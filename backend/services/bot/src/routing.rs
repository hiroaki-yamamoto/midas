use ::std::convert::TryFrom;
use ::std::sync::Arc;

use ::futures::TryFutureExt;
use ::http::StatusCode;
use ::warp::filters::BoxedFilter;
use ::warp::{Filter, Rejection, Reply};

use ::bot::entities::Bot;
use ::rpc::bot::Bot as RPCBot;
use ::rpc::status::Status;

use crate::context::Context;

pub fn construct(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  let register = ::warp::post()
    .and(::warp::filters::body::json())
    .map(move |bot: RPCBot| (bot, ctx.clone()))
    .untuple_one()
    .and_then(|bot: RPCBot, ctx: Arc<Context>| async move {
      let mut bot = Bot::try_from(bot).map_err(|e| {
        let code = StatusCode::EXPECTATION_FAILED;
        let status = Status::new(code.clone(), &e.to_string());
        return ::warp::reject::custom(status);
      })?;
      bot = ctx
        .transpiler
        .transpile(bot)
        .map_err(|e| {
          let code = StatusCode::INTERNAL_SERVER_ERROR;
          let status = Status::new(code.clone(), &e.to_string());
          return ::warp::reject::custom(status);
        })
        .await?;
      let _ = ctx
        .bot_repo
        .save(&[&bot])
        .map_err(|e| {
          let code = StatusCode::INTERNAL_SERVER_ERROR;
          let status = Status::new(code.clone(), &e.to_string());
          return ::warp::reject::custom(status);
        })
        .await?;
      return Ok::<Bot, Rejection>(bot);
    })
    .map(|bot: Bot| {
      let bot: RPCBot = bot.into();
      return ::warp::reply::json(&bot);
    });
  register.boxed()
}
