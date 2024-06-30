use ::async_trait::async_trait;
use ::mongodb::bson::doc;
use ::mongodb::Database;

#[async_trait]
pub trait DatabaseWriter {
  fn get_database(&self) -> &Database;
  fn get_col_name(&self) -> &str;
  async fn update_indices(&self, flds: &[&str]) {
    let col_name = self.get_col_name();
    let db = self.get_database();
    let has_index = db
      .run_command(doc! {"listIndexes": &col_name})
      .await
      .map(|item| {
        return item
          .get_document("listIndexes")
          .unwrap_or(&doc! {"ok": false})
          .get_bool("ok")
          .unwrap_or(false);
      })
      .unwrap_or(false);
    if !has_index {
      let mut indices = vec![];
      for fld_name in flds {
        indices.push(doc! { "name": format!("{}_index", fld_name), "key": doc!{
          *fld_name: 1,
        } })
      }
      let _ = db
        .run_command(doc! {
          "createIndexes": &col_name,
          "indexes": indices
        })
        .await;
    }
  }
}
