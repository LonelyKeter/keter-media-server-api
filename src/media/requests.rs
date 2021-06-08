use crate::req_prelude::*;
use super::types::*;

enum AuthorParam {
  Id(u64),
  Alias(String)
}

use rocket::form::{self, FromFormField, ValueField};

#[rocket::async_trait]
impl<'r> FromFormField<'r> for AuthorParam {
  fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
    todo!("Impl FromFormField for AuthorParam");
    unimplemented!();
  }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for MediaKind {
  fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
    todo!("Impl FromFormField for MediaKind");
    unimplemented!();
  }
}

type JsonOut<T> = rocket::serde::json::Json<T>;

#[get("/?<title>&<kind>&<author>&<rating>&<ord>", format = "json")]
pub async fn get_base(
  title: Option<String>,
  kind: Option<MediaKind>,
  author: Option<String>,
  rating: Option<u32>,
  ord: Option<Ordering>) 
    -> Json<Vec<MediaInfo>> {
  Json(vec![Media::new(12345)])
}

#[get("/<id>", format = "json")]
pub async fn get_concrete(id: u128) -> Json<Media> {
  unimplemented!();
}

#[post("/", format = "json", data = "<reg_media>")]
pub async fn post_media(reg_media: Json<u8>) {
  unimplemented!();
}

#[post("/materials?<media>&<author>", format = "json", data = "<reg_media>")]
pub async fn post_material_named(media: String, author: AuthorParam, reg_media: Json<MaterialInfo>) {
  unimplemented!();
}

#[post("/materials?<media>", format = "json", data = "<reg_media>")]
pub async fn post_material(media: u64, reg_media: Json<MaterialInfo>) {
  unimplemented!();
}

#[delete("/materials?<media>")]
pub async fn delete_material(media: u64) {
  unimplemented!();
}

#[put("/usage?<material>", format = "json", data = "<license>")]
pub async fn put_usage_material(material: u64, license: Json<License>) {
  unimplemented!();
}

#[put("/usage?<media>&<author>", format = "json", data = "<license>")]
pub async fn put_usage_media_named(media: String, author: AuthorParam, license: Json<License>) {
  unimplemented!();
}

#[put("/usage?<media>", format = "json", data = "<license>")]
pub async fn put_usage_media(media: u64, license: Json<License>) {
  unimplemented!();
}

#[get("/usage", format = "json")]
pub async fn get_usage() -> Json<Vec<Usage>> {
  unimplemented!();
}

#[get("/usage?<media>", format = "json")]
pub async fn get_usage_media(media: u64) -> Json<Vec<Usage>> {
  unimplemented!();
}

#[get("/usage?<media>&<author>", format = "json")]
pub async fn get_usage_media_named(media: String, author: AuthorParam) -> Json<Vec<Usage>> {
  unimplemented!();
}

#[get("/usage?<material>", format = "json")]
pub async fn get_usage_material(material: u64) -> Json<Vec<Usage>> {
  unimplemented!();
}