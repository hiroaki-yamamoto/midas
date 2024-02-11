use crate::bot_list::BotList;
use crate::bot_response::BotResponse;

impl BotList {
  pub fn new(bot_list: &[Box<BotResponse>]) -> Self {
    return Self {
      bots: bot_list.to_vec(),
    };
  }
}
