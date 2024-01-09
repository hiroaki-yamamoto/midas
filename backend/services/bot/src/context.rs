use ::std::sync::Arc;

use ::mongodb::Database;

use ::bot::{interfaces::IBotRepo, services::BotRepo};

pub struct Context {
  pub bot_repo: Arc<dyn IBotRepo + Send + Sync>,
}

impl Context {
  pub fn new(db: &Database) -> Self {
    return Self {
      bot_repo: Arc::new(BotRepo::new(db)),
    };
  }
}
