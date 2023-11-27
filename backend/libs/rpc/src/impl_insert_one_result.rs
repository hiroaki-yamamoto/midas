use ::bson::oid::ObjectId;

use super::insert_one_result::InsertOneResult;

impl From<Option<ObjectId>> for InsertOneResult {
  fn from(value: Option<ObjectId>) -> Self {
    return Self {
      id: value.map(|v| v.to_hex()).unwrap_or(String::default()),
    };
  }
}
