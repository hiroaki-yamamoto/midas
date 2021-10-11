use ::mongodb::Database;
use ::warp::filters::BoxedFilter;
use ::warp::{Filter, Reply};

// use ::rpc::rejection_handler::handle_rejection;
use ::bot::BotInfoRecorder;
use ::rpc::bot::Bot as RPCBot;

pub fn construct(db: &Database) -> BoxedFilter<(impl Reply,)> {
  let writer = BotInfoRecorder::new(db);
  let register = ::warp::post()
    .and(::warp::filters::body::json())
    .map(|bot: RPCBot| {});
}
