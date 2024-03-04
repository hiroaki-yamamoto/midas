use ::std::convert::TryFrom;
use ::std::sync::Arc;
use std::str::FromStr;

use ::futures::{StreamExt, TryFutureExt};
use ::http::StatusCode;
use ::log::error;
use ::mongodb::bson::oid::ObjectId;
use ::warp::filters::BoxedFilter;
use ::warp::{Filter, Rejection, Reply};

use ::bot::entities::Bot;
use ::bot::errors::BotInfoError;
use ::rpc::bot_get_request::BotGetRequest as RPCBotGetReq;
use ::rpc::bot_list::BotList;
use ::rpc::bot_request::BotRequest as RPCBotReq;
use ::rpc::bot_response::BotResponse as RPCBotResp;
use ::rpc::bot_status::BotStatus;
use ::rpc::status::Status;
use ::rpc::summary_detail::SummaryDetail;

use crate::context::Context;

pub fn construct(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  return get(ctx.clone())
    .or(list(ctx.clone()))
    .or(post(ctx.clone()))
    .or(put(ctx))
    .boxed();
}

fn get(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  let get = ::warp::path::param()
    .and(::warp::query())
    .and(::warp::get())
    .map(move |id: String, query: RPCBotGetReq| (id, query, ctx.clone()))
    .untuple_one()
    .and_then(
      |id: String, query: RPCBotGetReq, ctx: Arc<Context>| async move {
        let oid = ObjectId::from_str(&id).map_err(|e| {
          let code = StatusCode::EXPECTATION_FAILED;
          let status = Status::new(code.clone(), &e.to_string());
          return ::warp::reject::custom(status);
        })?;
        let bot = match *query.granularity {
          SummaryDetail::Detail => ctx.bot_repo.get_by_id(oid),
          SummaryDetail::Summary => ctx.bot_repo.summary_by_id(oid),
        }
        .await
        .map_err(|e| {
          let code = if let BotInfoError::ObjectNotFound(_) = e {
            StatusCode::NOT_FOUND
          } else {
            StatusCode::INTERNAL_SERVER_ERROR
          };
          let status = Status::new(code.clone(), &e.to_string());
          return ::warp::reject::custom(status);
        })?;
        return Ok::<Bot, Rejection>(bot);
      },
    )
    .map(|bot: Bot| {
      let bot: RPCBotResp = bot.into();
      return ::warp::reply::json(&bot);
    });
  return get.boxed();
}

fn post(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  let register = ::warp::post()
    .and(::warp::filters::body::json())
    .map(move |bot: RPCBotReq| (bot, ctx.clone()))
    .untuple_one()
    .and_then(|bot: RPCBotReq, ctx: Arc<Context>| async move {
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
        .save(&bot)
        .map_err(|e| {
          let code = StatusCode::INTERNAL_SERVER_ERROR;
          let status = Status::new(code.clone(), &e.to_string());
          return ::warp::reject::custom(status);
        })
        .await?;
      return Ok::<Bot, Rejection>(bot);
    })
    .map(|bot: Bot| {
      let bot: RPCBotResp = bot.into();
      return ::warp::reply::json(&bot);
    });
  return register.boxed();
}

fn put(ctx: Arc<Context>) -> BoxedFilter<(impl Reply,)> {
  let modify = ::warp::path::param()
    .and(::warp::put())
    .map(move |id: String| (id, ctx.clone()))
    .untuple_one()
    .and_then(|id: String, ctx: Arc<Context>| async move {
      let oid = ObjectId::from_str(&id).map_err(|e| {
        let code = StatusCode::EXPECTATION_FAILED;
        let status = Status::new(code.clone(), &e.to_string());
        return ::warp::reject::custom(status);
      })?;
      let bot = ctx.bot_repo.get_by_id(oid).await.map_err(|e| {
        let code = if let BotInfoError::ObjectNotFound(_) = e {
          StatusCode::NOT_FOUND
        } else {
          StatusCode::INTERNAL_SERVER_ERROR
        };
        let status = Status::new(code.clone(), &e.to_string());
        return ::warp::reject::custom(status);
      })?;
      if bot.status != BotStatus::Stopped {
        let code = StatusCode::EXPECTATION_FAILED;
        let status = Status::new(code.clone(), "Bot is not stopped");
        return Err(::warp::reject::custom(status));
      }
      return Ok((id, ctx));
    })
    .untuple_one()
    .and(::warp::filters::body::json())
    .and_then(|id: String, ctx: Arc<Context>, bot: RPCBotReq| async move {
      // Check ID, and then convert RPCBot to Bot that is used in the backend.
      if Some(id) == bot.id {
        let bot = Bot::try_from(bot).map_err(|e| {
          let code = StatusCode::EXPECTATION_FAILED;
          let status = Status::new(code.clone(), &e.to_string());
          return ::warp::reject::custom(status);
        })?;
        return Ok((bot, ctx));
      }
      let code = StatusCode::EXPECTATION_FAILED;
      let status = Status::new(code.clone(), "ID mismatch");
      return Err(::warp::reject::custom(status));
    })
    .untuple_one()
    .and_then(|bot: Bot, ctx: Arc<Context>| async move {
      let _ = ctx
        .bot_repo
        .save(&bot)
        .map_err(|e| {
          let code = StatusCode::INTERNAL_SERVER_ERROR;
          let status = Status::new(code.clone(), &e.to_string());
          return ::warp::reject::custom(status);
        })
        .await?;
      return Ok::<Bot, Rejection>(bot);
    })
    .map(|bot: Bot| {
      let bot: RPCBotResp = bot.into();
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
      let bots: Vec<Box<RPCBotResp>> = stream
        .filter_map(|bot_result| async move {
          if let Err(ref e) = bot_result {
            let e = e.clone();
            error!(error: err = e; "Failed to get bot info");
          }
          return bot_result.ok();
        })
        .map(|bot: Bot| {
          let bot: RPCBotResp = bot.into();
          return Box::new(bot);
        })
        .collect()
        .await;
      return Ok::<Vec<Box<RPCBotResp>>, Rejection>(bots);
    })
    .map(|bots: Vec<Box<RPCBotResp>>| {
      let bot_list = BotList::new(bots.as_slice());
      return ::warp::reply::json(&bot_list);
    });
  return list.boxed();
}
