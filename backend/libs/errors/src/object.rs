use ::err_derive::Error;

#[derive(Debug, Clone, Error)]
#[error(display = "Entity {} Not Found (Key: {:?})", entity, key)]
pub struct ObjectNotFound {
  entity: String,
  key: Option<String>,
}

impl ObjectNotFound {
  pub fn new<'lt>(entity: &str, key: impl Into<Option<&'lt str>>) -> Self {
    return Self {
      entity: entity.to_string(),
      key: key.into().map(|k| k.to_string()),
    };
  }
}
