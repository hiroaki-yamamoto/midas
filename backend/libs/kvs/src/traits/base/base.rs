use ::redis::AsyncCommands as Commands;

pub trait Base<T>
where
  T: Commands,
{
  fn __commands__(&self) -> T;
}
