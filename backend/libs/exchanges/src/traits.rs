use ::std::collections::HashMap;

use ::async_trait::async_trait;
use ::bytes::Bytes;
use ::chrono::{DateTime, Utc};
use ::futures::stream::{BoxStream, LocalBoxStream, Stream};
use ::mongodb::bson::{doc, oid::ObjectId, Document};
use ::mongodb::results::InsertManyResult;
use ::mongodb::Database;
use ::nats::asynk::Subscription;
use ::ring::hmac;
use ::serde::Serialize;

use ::types::GenericResult;
use types::ThreadSafeResult;

use crate::entities::APIKeyInternal;

use super::entities::{
  BookTicker, ExecutionResult, ExecutionType, Order, OrderInner, OrderOption,
};

use super::errors::ExecutionFailed;

#[async_trait]
pub trait TradeObserver {
  async fn start(&self) -> GenericResult<()>;
  async fn subscribe(&self) -> ::std::io::Result<BoxStream<'_, BookTicker>>;
}

pub(crate) trait TradeDateTime {
  fn symbol(&self) -> String;
  fn open_time(&self) -> DateTime<Utc>;
  fn close_time(&self) -> DateTime<Utc>;
}

pub trait Sign {
  fn get_secret_key(&self, prv_key: String) -> hmac::Key;
  fn sign(&self, body: String, prv_key: String) -> String {
    let secret = self.get_secret_key(prv_key);
    let tag = hmac::sign(&secret, body.as_bytes());
    let signature = Bytes::copy_from_slice(tag.as_ref());
    return format!("{:x}", signature);
  }
}

#[async_trait]
pub trait UserStream {
  async fn get_listen_key(&self, api_key: &APIKeyInternal)
    -> GenericResult<()>;
  async fn clise_listen_key(
    &self,
    api_key: &APIKeyInternal,
    listen_key: &String,
  ) -> GenericResult<()>;
  async fn start(&self) -> GenericResult<()>;
}
