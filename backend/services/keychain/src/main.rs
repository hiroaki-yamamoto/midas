use ::futures::FutureExt;
use ::futures::StreamExt;
use ::http::StatusCode;
use ::log::{as_display, info, warn};
use ::mongodb::bson::{doc, oid::ObjectId};
use ::std::convert::TryFrom;
use ::warp::Filter;

use ::access_logger::log;
use ::config::init;
use ::csrf::{CSRFOption, CSRF};
use ::keychain::{APIKey, IKeyChain, KeyChain};
use ::rpc::api_key::ApiKey as RPCAPIKey;
use ::rpc::api_key_list::ApiKeyList as RPCAPIKeyList;
use ::rpc::api_rename::ApiRename;
use ::rpc::insert_one_result::InsertOneResult;
use ::rpc::status::Status;
use ::warp_utils::handle_rejection;

macro_rules! declare_reject_func {
  () => {
    |e| {
      ::warp::reject::custom(Status::new(
        StatusCode::SERVICE_UNAVAILABLE,
        &format!("{}", e),
      ))
    }
  };
}

#[tokio::main]
async fn main() {
  init(|cfg, mut sig, db, broker, host| async move {
    let log_access = log();
    let keychain = KeyChain::new(&broker, db).await.unwrap();
    let path_param = ::warp::any()
      .map(move || {
        return (keychain.clone(),);
      })
      .untuple_one();

    let id_filter = ::warp::path::param().and_then(|id: String| async move {
      match ObjectId::parse_str(&id) {
        Err(_) => return Err(::warp::reject()),
        Ok(id) => return Ok(id),
      };
    });

    let get_handler = ::warp::get()
      .and(path_param.clone())
      .and_then(|keychain: KeyChain| async move {
        match keychain.list(doc! {}).await {
          Err(e) => {
            warn!(error = as_display!(e); "An error was occured when querying");
            return Err(::warp::reject());
          }
          Ok(cursor) => {
            return Ok(
              cursor
                .map(|mut api_key| {
                  api_key.inner_mut().prv_key = ("*").repeat(16);
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
      })
      .map(|api_key_list: Vec<RPCAPIKey>| {
        let api_key_list: Vec<Box<RPCAPIKey>> = api_key_list
          .into_iter()
          .map(|api_key| Box::new(api_key))
          .collect();
        return ::warp::reply::json(&RPCAPIKeyList { keys: api_key_list });
      });
    let post_handler = ::warp::post()
      .and(path_param.clone())
      .and(::warp::filters::body::json())
      .and_then(|keychain: KeyChain, api_key: RPCAPIKey| async move {
        let api_key: APIKey =
          APIKey::try_from(api_key).map_err(declare_reject_func!())?;
        return keychain
          .push(api_key)
          .await
          .map_err(declare_reject_func!())
          .map(|res| {
            let res: InsertOneResult = res.into();
            return res;
          });
      })
      .map(|res: InsertOneResult| {
        return ::warp::reply::json(&res);
      });
    let patch_handler = ::warp::patch()
      .and(path_param.clone())
      .and(id_filter)
      .and(::warp::filters::body::json())
      .and_then(
        |keychain: KeyChain, id: ObjectId, rename: ApiRename| async move {
          if let Err(_) = keychain.rename_label(id, &rename.label).await {
            return Err(::warp::reject());
          };
          return Ok(());
        },
      )
      .untuple_one()
      .map(|| ::warp::reply());
    let delete_handler = ::warp::delete()
      .and(path_param)
      .and(id_filter)
      .and_then(|keychain: KeyChain, id: ObjectId| async move {
        let del_defer = keychain.delete(id);
        if let Err(_) = del_defer.await {
          return Err(::warp::reject());
        };
        return Ok(());
      })
      .untuple_one()
      .map(|| ::warp::reply());
    let route = CSRF::new(CSRFOption::builder()).protect().and(
      get_handler
        .or(post_handler)
        .or(patch_handler)
        .or(delete_handler)
        .with(log_access)
        .recover(handle_rejection),
    );
    info!("Opened REST server on {}", host);
    let (_, ws_svr) = ::warp::serve(route)
      .tls()
      .cert_path(&cfg.tls.cert)
      .key_path(&cfg.tls.prv_key)
      .bind_with_graceful_shutdown(host, async move {
        sig.recv().await;
      });
    let svr = ws_svr.then(|_| async {
      warn!("REST Server is shutting down! Bye! Bye!");
    });
    svr.await;
  })
  .await;
}
