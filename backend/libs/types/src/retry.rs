pub use ::futures::Future;

pub async fn retry_async<F, O, T, E>(count: usize, func: F) -> Result<T, E>
where
  F: Fn() -> O,
  O: Future<Output = Result<T, E>>,
{
  let mut result = func().await;
  for _ in 1..count {
    result = func().await;
    if result.is_ok() {
      break;
    }
  }
  return result;
}
