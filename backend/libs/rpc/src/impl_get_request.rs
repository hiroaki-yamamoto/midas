use crate::bot_get_request::BotGetRequest;
use crate::summary_detail::SummaryDetail;

impl BotGetRequest {
  pub fn default_granularity() -> Box<SummaryDetail> {
    return Box::new(SummaryDetail::Summary);
  }
}
