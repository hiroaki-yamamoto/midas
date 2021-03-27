#[macro_export]
macro_rules! stateful_setter {
  ($fld_name: ident, $t: ty) => {
    pub fn $fld_name(self, value: $t) -> Self {
      let mut me = Self { ..self };
      me.$fld_name = value;
      return me;
    }
  };
}
