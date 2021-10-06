use ::rand::distributions::Alphanumeric;
use ::rand::thread_rng;
use ::rand::Rng;
use ::reqwest::cookie::Jar;
use ::reqwest::Client;

use ::types::ThreadSafeResult;

use super::entities::Bot;

static url: &str = "/transpiler";

pub struct Transpiler {
  cli: Client,
}

impl Transpiler {
  pub fn new(cli: Client) -> Self {
    return Self { cli };
  }

  pub async fn transpile(&self, bot: &Bot) -> ThreadSafeResult<()> {
    let token: String = thread_rng()
      .sample_iter(&Alphanumeric)
      .take(32)
      .map(char::from)
      .collect();
    let cookies = Jar::default();
    let cookie_url = "https://localhost".parse()?;
    cookies.add_cookie_str(&format!("XSRF-TOKEN={}", token), &cookie_url);
    self.cli.post(url);
    return Ok(());
  }
}
