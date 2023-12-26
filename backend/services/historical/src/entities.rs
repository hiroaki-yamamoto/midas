use ::rpc::history_fetch_request::HistoryFetchRequest;
use ::rpc::status_check_request::StatusCheckRequest;

use ::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SocketRequest {
  Fetch(HistoryFetchRequest),
  StatusCheck(StatusCheckRequest),
}
