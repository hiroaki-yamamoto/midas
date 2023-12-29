pub trait ISigner {
  fn sign(&self, body: String) -> String;
}
