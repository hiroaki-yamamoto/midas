use crate::bot::Bot;
use crate::bot_list::BotList;

impl BotList {
  pub fn new(bot_list: &[Box<Bot>]) -> Self {
    return Self {
      bots: bot_list.to_vec(),
    };
  }
}
