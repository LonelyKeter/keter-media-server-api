use rocket::fairing::AdHoc;
use rocket::form::{self, FromFormField, ValueField};
use rocket::serde::{json::Json};

use keter_media_model::{
  media::*,
  usage::*,
  userinfo::*
};

pub fn stage() -> AdHoc {
  AdHoc::on_ignite("MEDIA", |rocket| async {
    rocket.mount("/media", routes![
      get_base,
      
    ])
  })
}


pub enum AuthorParam {
  Id(std::num::NonZeroU64),
  Alias(String)
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for AuthorParam {
  fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
    if let Ok(id) = str::parse::<UserKey>(field.value) {
      Ok(AuthorParam::Id(id))
    } else {
      Ok(AuthorParam::Alias(field.value.to_owned()))
    }
  }
}

#[derive(FromFormField)]
pub enum MediaKindParam {
  Audio,
  Video,
  Image
}

impl From<MediaKindParam> for MediaKind {
  fn from(other: MediaKindParam) -> MediaKind {
    match other {
        MediaKindParam::Audio => MediaKind::Audio,
        MediaKindParam::Video => MediaKind::Video,
        MediaKindParam::Image => MediaKind::Image,
    }
  }
}

type JsonOut<T> = rocket::serde::json::Json<T>;

#[get("/?<title>&<kind>&<author>&<rating>", format = "json")]
pub async fn get_base(title: Option<String>, kind: Option<MediaKindParam>, author: Option<AuthorParam>, rating: Option<f64>) 
    -> Json<Vec<MediaInfo>> {
      unimplemented!()
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