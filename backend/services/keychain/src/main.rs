use ::std::net::SocketAddr;

use ::clap::Clap;
use ::futures::FutureExt;
use ::futures::StreamExt;
use ::libc::{SIGINT, SIGTERM};
use ::mongodb::bson::{doc, oid::ObjectId};
use ::mongodb::Client;
use ::slog::Logger;
use ::tokio::signal::unix as signal;
use ::warp::Filter;

use ::config::{CmdArgs, Config};
use ::csrf::{CSRFOption, CSRF};
use ::exchanges::{APIKey, KeyChain};
use ::rpc::entities::Exchanges;
use ::rpc::keychain::ApiRename;
use ::rpc::keychain::{ApiKey as RPCAPIKey, ApiKeyList as RPCAPIKeyList};
use warp::reply;

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

  let get_handler = ::warp::get()
    .and(path_param.clone())
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
                .map(|api_key| {
                  let api_key: Result<RPCAPIKey, String> = api_key.into();
                  return api_key;
                })
                .filter_map(|api_key_result| async move { api_key_result.ok() })
                .collect::<Vec<RPCAPIKey>>()
                .await,
            );
          }
        };
      },
    )
    .map(|api_key_list| {
      return ::warp::reply::json(&RPCAPIKeyList { keys: api_key_list });
    });
  let post_handler = ::warp::post()
    .and(path_param.clone())
    .and(::warp::filters::body::json())
    .and_then(
      |exchange: Exchanges,
       keychain: KeyChain,
       _: Logger,
       api_key: RPCAPIKey| async move {
        let mut api_key: APIKey = api_key.into();
        api_key.exchange = exchange.as_string();
        let _ = keychain.write(api_key).await;
        return Result::<(), ::std::convert::Infallible>::Ok(());
      },
    )
    .untuple_one()
    .map(|| {
      return reply();
    });
  let patch_handler = ::warp::patch()
    .and(path_param.clone().and(::warp::path::param().and_then(
      |id: String| async move {
        match ObjectId::with_string(&id) {
          Err(_) => return Err(::warp::reject()),
          Ok(id) => return Ok(id),
        };
      },
    )))
    .and(::warp::filters::body::json())
    .and_then(
      |_: Exchanges,
       keychain: KeyChain,
       _: Logger,
       id: ObjectId,
       rename: ApiRename| async move {
        if let Err(_) = keychain.rename_label(id, &rename.label).await {
          return Err(::warp::reject());
        };
        return Ok(());
      },
    )
    .untuple_one()
    .map(|| ::warp::reply());
  let delete_handler = ::warp::delete()
    .and(path_param.and(::warp::path::param().and_then(
      |id: String| async move {
        match ObjectId::with_string(&id) {
          Err(_) => return Err(::warp::reject()),
          Ok(id) => return Ok(id),
        };
      },
    )))
    .and_then(
      |exc: Exchanges, keychain: KeyChain, _: Logger, id: ObjectId| async move {
        let del_defer =
          keychain.delete(doc! {"_id": id, "exchange": exc.as_string()});
        if let Err(_) = del_defer.await {
          return Err(::warp::reject());
        };
        return Ok(());
      },
    )
    .untuple_one()
    .map(|| ::warp::reply());
  let route = CSRF::new(CSRFOption::builder()).protect().and(
    get_handler
      .or(post_handler)
      .or(patch_handler)
      .or(delete_handler),
  );
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
