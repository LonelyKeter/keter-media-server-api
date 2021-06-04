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
  Json(Vec::new())
}

#[get("/<id>", format = "json")]
pub async fn get_concrete(id: u128) -> Json<Media> {
  unimplemented!();
}

#[post("/media", format = "json", data = "<reg_media>")]
pub async fn post_media(reg_media: Json<u8>) {
  unimplemented!();
}

#[post("/media/materials?<media>&<author>", format = "json", data = "<reg_media>")]
pub async fn post_material_named(
  media: String, 
  author: AuthorParam, 
  reg_media: Json<MaterialInfo>) {
  unimplemented!();
}

#[post("/media/materials?<media>", format = "json", data = "<reg_media>")]
pub async fn post_material(media: u64, reg_media: Json<MaterialInfo>) {
  unimplemented!();
}

#[delete("/media/materials?<media>")]
pub async fn delete_material(media: u64) {
  unimplemented!();
}

#[put("/media/usage?<material>", format = "json", data = "<license>")]
pub async fn put_usage_material(material: u64, license: Json<License>) {
  unimplemented!();
}

#[put("/media/usage?<media>&<author>", format = "json", data = "<license>")]
pub async fn put_usage_media_named(media: String, author: AuthorParam, license: Json<License>) {
  unimplemented!();
}

#[put("/media/usage?<media>", format = "json", data = "<license>")]
pub async fn put_usage_media(media: u64, license: Json<License>) {
  unimplemented!();
}

#[get("/media/usage", format = "json")]
pub async fn get_usage() -> Json<Vec<Usage>> {
  unimplemented!();
}

#[get("/media/usage?<media>", format = "json")]
pub async fn get_usage_media(media: u64) -> Json<Vec<Usage>> {
  unimplemented!();
}

#[get("/media/usage?<media>&<author>", format = "json")]
pub async fn get_usage_media_named(media: String, author: AuthorParam) -> Json<Vec<Usage>> {
  unimplemented!();
}

#[get("/media/usage?<material>", format = "json")]
pub async fn get_usage_material(material: u64) -> Json<Vec<Usage>> {
  unimplemented!();
}