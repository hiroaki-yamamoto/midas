use ::clap::Clap;
use ::futures::StreamExt;
use ::mongodb::bson::doc;
use ::mongodb::Client;
use ::warp::Filter;
use slog::Logger;

use ::config::{CmdArgs, Config};
use ::exchanges::{APIKey, KeyChain};
use ::rpc::entities::Exchanges;

#[tokio::main]
async fn main() {
  let opts: CmdArgs = CmdArgs::parse();
  let config = Config::from_fpath(Some(opts.config)).unwrap();
  let (logger, _) = config.build_slog();
  let db_cli = Client::with_uri_str(&config.db_url).await.unwrap();
  let db = db_cli.database("midas");
  let keychain = KeyChain::new(db).await;

  let get_handler = ::warp::get()
    .and(::warp::path::param())
    .and_then(|exchange: String| async move {
      let exchange: Exchanges = match exchange.parse() {
        Err(_) => return Err(::warp::reject::not_found()),
        Ok(v) => v,
      };
      return Ok(exchange);
    })
    .map(|exchange| {
      return (exchange, keychain.clone(), logger.clone());
    })
    .untuple_one()
    .and_then(
      |exchange: Exchanges, keychain: KeyChain, logger: Logger| async move {
        match keychain
          .list(doc! {
            "exchange": exchange.as_string(),
          })
          .await
        {
          Err(e) => {
            ::slog::warn!(
              logger,
              "An error was occured when querying: {}",
              e;
              "exchange" => exchange.as_string()
            );
            return Err(::warp::reject());
          }
          Ok(cursor) => {
            return Ok(cursor.collect::<Vec<APIKey>>().await);
          }
        };
      },
    )
    .map(|api_key_list| {});
}
