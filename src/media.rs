use rocket::fairing::AdHoc;
use rocket::form::{self, FromFormField, ValueField};
use rocket::serde::{json::Json};

use keter_media_model::{
  media::*,
  usage::*,
  userinfo::*
};

use crate::auth::*;

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
pub async fn get_concrete(id: MediaKey) -> Json<Media> {
  unimplemented!();
}

#[post("/", format = "json", data = "<reg_media>")]
pub async fn post_media(
  reg_media: Json<u8>, 
  auth: &Authentication, author: Author) {
  unimplemented!();
}

#[post("/materials?<media>", format = "json", data = "<reg_media>")]
pub async fn post_material_named(
  media: String, reg_media: Json<MaterialInfo>,
  auth: &Authentication, author: Author) {
  unimplemented!();
}

#[post("/materials?<media>", format = "json", data = "<reg_media>")]
pub async fn post_material(
  media: MediaKey, reg_media: Json<MaterialInfo>,
  auth: &Authentication, author: Author) {
  unimplemented!();
}

//TODO: multiple routes for admin and author
#[delete("/materials?<id>")]
pub async fn delete_material_admin(
  id: MaterialKey,
  auth: &Authentication, admin: Admin) {
  unimplemented!();
}

#[delete("/materials?<id>")]
pub async fn delete_material_author(
  id: MaterialKey,
  auth: &Authentication, author: Author) {
  unimplemented!();
}

#[put("/usage?<material>", format = "json", data = "<license>")]
pub async fn put_usage_material(
  material: MaterialKey, license: Json<License>,
  auth: &Authentication, author: Author) {
  unimplemented!();
}

#[put("/usage?<media>", format = "json", data = "<license>")]
pub async fn put_usage_media_named(
  media: String, license: Json<License>,
  auth: &Authentication, author: Author) {
  unimplemented!();
}

#[put("/usage?<media>", format = "json", data = "<license>")]
pub async fn put_usage_media(
  media: MediaKey, license: Json<License>,
  auth: &Authentication, author: Author) {
  unimplemented!();
}

//TODO: Separate usage for admin and author
#[get("/usage", format = "json")]
pub async fn get_usage(auth: &Authentication, author: Author) -> Json<Vec<Usage>> {
  unimplemented!();
}

//TODO: Separate usage for admin and author
#[get("/usage?<media>", format = "json")]
pub async fn get_usage_media(
  media: MediaKey,
  auth: &Authentication, author: Author) -> Json<Vec<Usage>> {
  unimplemented!();
}

//TODO: Separate usage for admin and author
#[get("/usage?<media>", format = "json")]
pub async fn get_usage_media_named(
  media: String,
  auth: &Authentication, author: Author) -> Json<Vec<Usage>> {
  unimplemented!();
}

//TODO: Separate usage for admin and author
#[get("/usage?<material>", format = "json")]
pub async fn get_usage_material(
  material: MaterialKey,
  auth: &Authentication, author: Author) -> Json<Vec<Usage>> {
  unimplemented!();
}