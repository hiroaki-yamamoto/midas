mod entities;

use ::std::net::SocketAddr;

use ::clap::Clap;
use ::futures::FutureExt;
use ::futures::StreamExt;
use ::libc::{SIGINT, SIGTERM};
use ::mongodb::bson::doc;
use ::mongodb::Client;
use ::slog::Logger;
use ::tokio::signal::unix as signal;
use ::warp::Filter;

use ::config::{CmdArgs, Config};
use ::exchanges::{APIKey, KeyChain};
use ::rpc::entities::Exchanges;

use self::entities::APIKeyList;

#[tokio::main]
async fn main() {
  let opts: CmdArgs = CmdArgs::parse();
  let config = Config::from_fpath(Some(opts.config)).unwrap();
  let (logger, _) = config.build_slog();
  let logger_in_handler = logger.clone();
  let db_cli = Client::with_uri_str(&config.db_url).await.unwrap();
  let db = db_cli.database("midas");
  let keychain = KeyChain::new(db).await;

  let path_param = ::warp::path::param()
    .and_then(|exchange: String| async move {
      let exchange: Exchanges = match exchange.parse() {
        Err(_) => return Err(::warp::reject::not_found()),
        Ok(v) => v,
      };
      return Ok(exchange);
    })
    .map(move |exchange| {
      return (exchange, keychain.clone(), logger_in_handler.clone());
    })
    .untuple_one();

  let get_handler = path_param
    .clone()
    .and(::warp::get())
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
            return Ok(
              cursor
                .map(|mut api_key| {
                  api_key.prv_key = ("*").repeat(16);
                  return api_key;
                })
                .collect::<Vec<APIKey<String>>>()
                .await,
            );
          }
        };
      },
    )
    .map(|api_key_list| {
      return ::warp::reply::json(&APIKeyList { keys: api_key_list });
    });
  let post_handler = path_param
    .and(::warp::post())
    .and(::warp::filters::body::json())
    .map(
      |exchanges: Exchanges,
       keychain: KeyChain,
       logger: Logger,
       api_key: APIKey<String>| {},
    );
  let route = get_handler;
  let mut sig =
    signal::signal(signal::SignalKind::from_raw(SIGTERM | SIGINT)).unwrap();
  let host: SocketAddr = config.host.parse().unwrap();
  ::slog::info!(logger, "Opened REST server on {}", host);
  let (_, ws_svr) = ::warp::serve(route)
    .tls()
    .cert_path(&config.tls.cert)
    .key_path(&config.tls.prv_key)
    .bind_with_graceful_shutdown(host, async move {
      sig.recv().await;
    });
  let svr = ws_svr.then(|_| async {
    ::slog::warn!(logger, "REST Server is shutting down! Bye! Bye!");
  });
  svr.await;
}
