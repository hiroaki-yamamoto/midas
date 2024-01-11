use ::std::sync::Arc;

use ::async_trait::async_trait;
use ::random::generate_random_txt;
use ::reqwest::header;
use ::reqwest::Client;

use ::errors::{HTTPResult, StatusFailure};

use crate::entities::Bot;
use crate::interfaces::ITranspiler;

pub struct Transpiler {
  cli: Client,
  location: Arc<str>,
}

impl Transpiler {
  pub fn new(cli: Client, location: &str) -> Self {
    return Self {
      cli,
      location: Arc::from(location),
    };
  }
}

#[async_trait]
impl ITranspiler for Transpiler {
  async fn transpile(&self, bot: Bot) -> HTTPResult<Bot> {
    let token: String = generate_random_txt(32);
    let resp = self
      .cli
      .post(self.location.as_ref())
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
