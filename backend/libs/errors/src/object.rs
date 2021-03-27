use ::err_derive::Error;

#[derive(Debug, Clone, Error)]
#[error(display = "Entity {} Not Found", entity)]
pub struct ObjectNotFound {
  entity: String,
}

impl ObjectNotFound {
  pub fn new(entity: String) -> Self {
    return Self { entity };
  }
}
