use ::std::convert::TryFrom;

use ::mongodb::Database;
use ::warp::filters::BoxedFilter;
use ::warp::{Filter, Reply};

use ::bot::entities::Bot;
use ::bot::BotInfoRecorder;
use ::rpc::bot::Bot as RPCBot;

pub fn construct(db: &Database) -> BoxedFilter<(impl Reply,)> {
  let writer = BotInfoRecorder::new(db);
  let register = ::warp::post()
    .and(::warp::filters::body::json())
    .map(move |bot: RPCBot| (bot, &writer))
    .untuple_one()
    .and_then(|bot: RPCBot, writer: &BotInfoRecorder| async move {
      let bot = Bot::try_from(bot);
      // ???
      writer.write(&bot).await;
      return Ok(());
    })
    .map(|_| {
      return ::warp::reply();
    });
  register.boxed()
}
