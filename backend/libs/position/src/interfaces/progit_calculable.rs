use ::rug::Float;

/// Note: This trait is special. It should be implemented for **entities** while
/// the most of the interfaces are implemented for **services**.
pub trait IProfitCalculable {
  fn get_orig_amount(&self) -> Float;
}
