use ::rug::Float;

#[derive(Debug, Clone)]
pub struct OrderInner {
  pub price: Float,
  pub qty: Float,
}

impl Default for OrderInner {
  fn default() -> Self {
    return Self {
      price: Float::with_val(32, 0.0),
      qty: Float::with_val(32, 0.0),
    };
  }
}

impl AsRef<Self> for OrderInner {
  fn as_ref(&self) -> &Self {
    return self;
  }
}

impl<T> ::std::ops::Add<T> for OrderInner
where
  T: AsRef<Self>,
{
  type Output = Self;
  fn add(self, rhs: T) -> Self::Output {
    let rhs = rhs.as_ref();
    return Self {
      qty: self.qty.clone() + &rhs.qty,
      price: ((self.qty.clone() * &self.price)
        + (rhs.qty.clone() * &rhs.price))
        / (self.qty.clone() + &rhs.qty),
    };
  }
}

impl<T> ::std::ops::AddAssign<T> for OrderInner
where
  T: AsRef<Self>,
{
  fn add_assign(&mut self, rhs: T) {
    let rhs = rhs.as_ref();
    self.price =
      (self.qty.clone() * &self.price) + (rhs.qty.clone() * &rhs.price);
    self.price = &self.price / (self.qty.clone() + &rhs.qty);
    self.qty = self.qty.clone() + &rhs.qty;
  }
}

#[derive(Debug, Clone, Default)]
pub struct Order {
  pub symbol: String,
  pub inner: Vec<OrderInner>,
}

impl Order {
  pub fn new(symbol: String, inner: Vec<OrderInner>) -> Self {
    return Self { symbol, inner };
  }

  pub fn sum(&self) -> OrderInner {
    return self.inner.clone().into_iter().fold(
      OrderInner {
        price: Float::with_val(32, 0.0),
        qty: Float::with_val(32, 0.0),
      },
      |acc, item| acc + item,
    );
  }
}
