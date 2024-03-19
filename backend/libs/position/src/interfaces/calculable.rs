use ::rug::Float;

/// Note: This trait is special. It should be implemented for **entities** while
/// the most of the interfaces are implemented for **services**.
pub trait Calculable {
  fn get_orig_amount(&self) -> Float;
  fn get_executed_amount(&self) -> Float;
  fn get_price(&self) -> Float;
}
