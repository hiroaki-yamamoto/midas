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
      qty: Float::with_val(32, &self.qty + &rhs.qty),
      price: (Float::with_val(32, &self.qty * &self.price)
        + Float::with_val(32, &rhs.qty * &rhs.price))
        / Float::with_val(32, &self.qty + &rhs.qty),
    };
  }
}

impl<T> ::std::ops::AddAssign<T> for OrderInner
where
  T: AsRef<Self>,
{
  fn add_assign(&mut self, rhs: T) {
    let rhs = rhs.as_ref();
    self.price = Float::with_val(32, &self.qty * &self.price)
      + Float::with_val(32, &rhs.qty * &rhs.price);
    self.price = &self.price / Float::with_val(32, &self.qty + &rhs.qty);
    self.qty = Float::with_val(32, &self.qty + &rhs.qty);
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
