use ::async_trait::async_trait;

#[async_trait]
pub trait HistoryRecorder {
  async fn spawn(&self);
}
