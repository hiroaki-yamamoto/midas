use ::std::sync::Arc;

use ::mongodb::Database;
use ::reqwest::Client;

use ::bot::{
  interfaces::{IBotRepo, ITranspiler},
  services::{BotRepo, Transpiler},
};

pub struct Context {
  pub bot_repo: Arc<dyn IBotRepo + Send + Sync>,
  pub transpiler: Arc<dyn ITranspiler + Send + Sync>,
}

impl Context {
  pub fn new(db: &Database, cli: Client, transpiler_location: &str) -> Self {
    return Self {
      bot_repo: Arc::new(BotRepo::new(db)),
      transpiler: Arc::new(Transpiler::new(cli, transpiler_location)),
    };
  }
}
