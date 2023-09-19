use ::rand::distributions::Alphanumeric;
use ::rand::thread_rng;
use ::rand::Rng;
use ::reqwest::header;
use ::reqwest::Client;

use ::errors::{HTTPResult, StatusFailure};

use super::entities::Bot;

pub struct Transpiler {
  cli: Client,
  location: String,
}

impl Transpiler {
  pub fn new(cli: Client, location: String) -> Self {
    return Self { cli, location };
  }

  pub async fn transpile(&self, bot: &Bot) -> HTTPResult<Bot> {
    let token: String = thread_rng()
      .sample_iter(&Alphanumeric)
      .take(32)
      .map(char::from)
      .collect();
    let resp = self
      .cli
      .post(self.location.to_owned())
      .header("X-XSRF-TOKEN", &token)
      .header(header::COOKIE, &format!("XSRF-TOKEN={}", token))
      .body(bot.cond_ts.clone())
      .send()
      .await?;
    if let Err(_) = resp.error_for_status_ref() {
      let resp_url = resp.url().clone();
      let status = resp.status().as_u16();
      let text = resp.text().await.unwrap_or(String::default());
      return Err(
        StatusFailure::new(Some(resp_url.into()), status, String::from(text))
          .into(),
      );
    }
    let mut bot_js = bot.clone();
    bot_js.cond_js = resp.text().await.ok();
    return Ok(bot_js);
  }
}
