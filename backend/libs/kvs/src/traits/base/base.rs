use ::redis::AsyncCommands as Commands;

pub trait Base<T>
where
  T: Commands,
{
  fn commands(&self) -> T;
}
