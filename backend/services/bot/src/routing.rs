use ::std::convert::TryFrom;
use ::std::sync::Arc;

use ::futures::{StreamExt, TryFutureExt};
use ::http::StatusCode;
use ::log::{as_error, error};
use ::warp::filters::BoxedFilter;
use ::warp::{Filter, Rejection, Reply};

use ::bot::entities::Bot;
use ::rpc::bot::Bot as RPCBot;
use ::rpc::bot_list::BotList;
use ::rpc::status::Status;

use crate::context::Context;

pub fn construct(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  return list(ctx.clone()).or(post(ctx.clone())).or(put(ctx)).boxed();
}

fn post(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
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
  return register.boxed();
}

fn put(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  let modify = ::warp::path::param()
    .and(::warp::put())
    .and(::warp::filters::body::json())
    .and_then(|id: String, bot: RPCBot| async move {
      // Check ID, and then convert RPCBot to Bot that is used in the backend.
      if Some(id) == bot.id {
        let bot = Bot::try_from(bot).map_err(|e| {
          let code = StatusCode::EXPECTATION_FAILED;
          let status = Status::new(code.clone(), &e.to_string());
          return ::warp::reject::custom(status);
        })?;
        return Ok(bot);
      }
      let code = StatusCode::EXPECTATION_FAILED;
      let status = Status::new(code.clone(), "ID mismatch");
      return Err(::warp::reject::custom(status));
    })
    .map(move |bot: Bot| (bot, ctx.clone()))
    .untuple_one()
    .and_then(|bot: Bot, ctx: Arc<Context>| async move {
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
  return modify.boxed();
}

fn list(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  let list = ::warp::get()
    .and(::warp::path::end())
    .map(move || ctx.clone())
    .and_then(|ctx: Arc<Context>| async move {
      let stream = ctx
        .bot_repo
        .list()
        .map_err(|e| {
          let code = StatusCode::INTERNAL_SERVER_ERROR;
          let status = Status::new(code.clone(), &e.to_string());
          return ::warp::reject::custom(status);
        })
        .await?;
      let bots: Vec<Box<RPCBot>> = stream
        .filter_map(|bot_result| async move {
          if let Err(ref e) = bot_result {
            let e = e.clone();
            error!(error = as_error!(e); "Failed to get bot info");
          }
          return bot_result.ok();
        })
        .map(|bot: Bot| {
          let bot: RPCBot = bot.into();
          return Box::new(bot);
        })
        .collect()
        .await;
      return Ok::<Vec<Box<RPCBot>>, Rejection>(bots);
    })
    .map(|bots: Vec<Box<RPCBot>>| {
      let bot_list = BotList::new(bots.as_slice());
      return ::warp::reply::json(&bot_list);
    });
  return list.boxed();
}
