#[macro_export]
macro_rules! stateful_setter {
  ($fld_name: ident, $t: ty) => {
    pub fn $fld_name(&mut self, value: $t) -> &mut Self {
      self.$fld_name = value;
      return self;
    }
  };
}
