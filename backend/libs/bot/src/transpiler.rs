use ::rand::distributions::Alphanumeric;
use ::rand::thread_rng;
use ::rand::Rng;
use ::reqwest::header;
use ::reqwest::Client;

use ::errors::StatusFailure;
use ::types::ThreadSafeResult;

use super::entities::Bot;

pub struct Transpiler {
  cli: Client,
}

impl Transpiler {
  const LOCATION: &'static str = "https://transpiler_service:50000";
  pub fn new(cli: Client) -> Self {
    return Self { cli };
  }

  pub async fn transpile(&self, bot: &Bot) -> ThreadSafeResult<Bot> {
    let token: String = thread_rng()
      .sample_iter(&Alphanumeric)
      .take(32)
      .map(char::from)
      .collect();
    let resp = self
      .cli
      .post(Self::LOCATION)
      .header("X-XSRF-TOKEN", &token)
      .header(header::COOKIE, &format!("XSRF-TOKEN={}", token))
      .body(bot.cond_ts.clone())
      .send()
      .await?;
    if let Err(_) = resp.error_for_status_ref() {
      let resp_url = resp.url().clone();
      let status = resp.status().as_u16();
      let text = resp.text().await.unwrap_or(String::default());
      return Err(Box::new(StatusFailure::new(
        Some(resp_url),
        status,
        String::from(text),
      )));
    }
    let mut bot_js = bot.clone();
    bot_js.cond_js = resp.text().await.ok();
    return Ok(bot_js);
  }
}
