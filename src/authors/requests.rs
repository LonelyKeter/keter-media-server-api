use crate::req_prelude::*;
use super::types::*;

#[get("/", format = "json")]
pub async fn authors() -> Json<Vec<Author>> {
  Json(vec![Author::new(12345)])
}
