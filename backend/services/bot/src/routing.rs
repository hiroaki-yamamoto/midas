use ::std::convert::TryFrom;

use ::http::StatusCode;
use ::mongodb::Database;
use ::warp::filters::BoxedFilter;
use ::warp::{Filter, Reply};

use ::bot::entities::Bot;
use ::bot::interfaces::IBotRepo;
use ::bot::services::BotRepo;
use ::bot::Transpiler;
use ::reqwest::Client;
use ::rpc::bot::Bot as RPCBot;
use ::rpc::status::Status;

pub fn construct(
  db: &Database,
  cli: Client,
  transpiler_location: &str,
) -> BoxedFilter<(impl Reply,)> {
  let writer = BotRepo::new(db);
  let t_loc: String = transpiler_location.into();
  let register = ::warp::post()
    .and(::warp::filters::body::json())
    .map(move |bot: RPCBot| (bot, cli.clone(), writer.clone(), t_loc.clone()))
    .untuple_one()
    .and_then(
      |bot: RPCBot,
       cli: Client,
       writer: BotRepo,
       transpiler_location: String| async move {
        let bot = Bot::try_from(bot);
        if let Err(e) = bot {
          let code = StatusCode::EXPECTATION_FAILED;
          let status = Status::new(code.clone(), &e.to_string());
          return Err(::warp::reject::custom(status));
        }
        let transpiler = Transpiler::new(cli, transpiler_location);
        let bot = transpiler.transpile(&bot.unwrap()).await;
        let bot = match bot {
          Err(e) => {
            let code = StatusCode::INTERNAL_SERVER_ERROR;
            let status = Status::new(code.clone(), &e.to_string());
            return Err(::warp::reject::custom(status));
          }
          Ok(bot) => bot,
        };
        return match writer.save(&[&bot]).await {
          Ok(_) => Ok(bot),
          Err(e) => {
            let code = StatusCode::INTERNAL_SERVER_ERROR;
            let status = Status::new(code.clone(), &e.to_string());
            return Err(::warp::reject::custom(status));
          }
        };
      },
    )
    .map(|bot: Bot| {
      let bot: RPCBot = bot.into();
      return ::warp::reply::json(&bot);
    });
  register.boxed()
}
