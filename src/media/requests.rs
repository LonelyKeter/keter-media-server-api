use crate::req_prelude::*;
use super::types::*;

#[get("/", format = "json")]
pub async fn media_base() -> Json<Vec<Media>> {
  Json(vec![Media::new(12345)])
}