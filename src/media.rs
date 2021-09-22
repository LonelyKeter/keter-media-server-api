use keter_media_db::db::model::MediaSearchKey;
use rocket::fairing::AdHoc;
use rocket::form::{self, FromFormField, ValueField};
use rocket::response::status::{Accepted, NotFound};
use rocket::serde::json::Json;

use keter_media_model::{media::*, usage::*, userinfo::*};

use crate::{auth::*, utility::*};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("MEDIA", |rocket| async {
        rocket.mount(
            "/api/media",
            routes![
                get,
                get_media_id,
                get_media_author_id,
                get_materials,
                get_material_id,
                get_material_id_used,
                get_reviews,
                put_review
            ],
        )
    })
}

pub enum AuthorParam {
    Id(UserKey),
    Alias(String),
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
    Image,
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

#[get("/")]
pub async fn get(user: Unauthenticated) -> JsonApiResponce<Vec<MediaInfo>, ()> {
    JsonApiResponce::get_many(user.privelegies().get_media_many().await)
}

#[get("/?<title>&<kind>&<author>&<rating>", format = "json")]
pub async fn get_base(
    title: Option<String>,
    kind: Option<MediaKindParam>,
    author: Option<AuthorParam>,
    rating: Option<MediaRating>,
) -> Json<Vec<MediaInfo>> {
    unimplemented!()
}

#[get("/<id>", format = "json")]
pub async fn get_media_id(id: MediaKey, user: Unauthenticated) -> JsonApiResponce<MediaInfo, ()> {
    JsonApiResponce::get_opt(user.privelegies().get_media_id(id).await)
}

#[get("/?<author_id>", format = "json")]
pub async fn get_media_author_id(
    author_id: UserKey,
    user: Unauthenticated,
) -> JsonApiResponce<Vec<MediaInfo>, ()> {
    JsonApiResponce::get_many(user.privelegies().get_media_author_id(author_id).await)
}

#[get("/<id>/reviews", format = "json")]
pub async fn get_reviews(
    id: MediaKey,
    user: Unauthenticated,
) -> JsonApiResponce<Vec<UserReview>, ()> {
    JsonApiResponce::get_many(user.privelegies().get_reviews(MediaSearchKey::Id(id)).await)
}

#[put("/<id>/reviews", format = "json", data = "<review>")]
pub async fn put_review(
    id: MediaKey,
    review: Json<Review>,
    user: Registered,
) -> JsonApiResponce<(), ()> {
    JsonApiResponce::get(
        user.privelegies()
            .post_review(&MediaSearchKey::Id(id), &review.0)
            .await,
    )
}

#[post("/", format = "json", data = "<reg_media>")]
pub async fn post_media(reg_media: Json<u8>, auth: &Authentication, author: Author) {
    unimplemented!();
}

#[get("/<media_id>/materials", format = "json")]
pub async fn get_materials(
    media_id: MediaKey,
    user: Unauthenticated,
) -> JsonApiResponce<Vec<MaterialInfo>, ()> {
    JsonApiResponce::get_many(user.privelegies().get_materials(media_id).await)
}

#[get("/materials/<id>", format = "json", rank = 1)]
pub async fn get_material_id(
    id: MaterialKey,
    user: Unauthenticated,
) -> JsonApiResponce<MaterialInfo, ()> {
    JsonApiResponce::get_opt(user.privelegies().get_material_id(id).await)
}

#[get("/materials/<id>?used", format = "json", rank = 2)]
pub async fn get_material_id_used(
    id: MaterialKey,
    auth: &Authentication,
    user: Registered,
) -> JsonApiResponce<bool, ()> {
    JsonApiResponce::get(user.privelegies().is_material_used(id).await)
}

#[post("/materials?<media>", format = "json", data = "<reg_media>")]
pub async fn post_material(
    media: MediaKey,
    reg_media: Json<MaterialInfo>,
    auth: &Authentication,
    author: Author,
) {
    unimplemented!();
}

//TODO: multiple routes for admin and author
#[delete("/materials?<id>")]
pub async fn delete_material_admin(id: MaterialKey, auth: &Authentication, admin: Admin) {
    unimplemented!();
}

#[delete("/materials?<id>")]
pub async fn delete_material_author(id: MaterialKey, auth: &Authentication, author: Author) {
    unimplemented!();
}

#[put("/usage?<material>", format = "json", data = "<license>")]
pub async fn put_usage_material(
    material: MaterialKey,
    license: Json<License>,
    auth: &Authentication,
    author: Author,
) {
    unimplemented!();
}

#[put("/usage?<media>", format = "json", data = "<license>")]
pub async fn put_usage_media_named(
    media: String,
    license: Json<License>,
    auth: &Authentication,
    author: Author,
) {
    unimplemented!();
}

#[put("/usage?<media>", format = "json", data = "<license>")]
pub async fn put_usage_media(
    media: MediaKey,
    license: Json<License>,
    auth: &Authentication,
    author: Author,
) {
    unimplemented!();
}

/*

//TODO: Separate usage for admin and author
#[get("/usage", format = "json")]
pub async fn get_usage(auth: &Authentication, author: Author) -> Json<Vec<Usage>> {
    unimplemented!();
}

//TODO: Separate usage for admin and author
#[get("/usage?<media>", format = "json")]
pub async fn get_usage_media(
    media: MediaKey,
    auth: &Authentication,
    author: Author,
) -> Json<Vec<Usage>> {
    unimplemented!();
}

//TODO: Separate usage for admin and author
#[get("/usage?<media>", format = "json")]
pub async fn get_usage_media_named(
    media: String,
    auth: &Authentication,
    author: Author,
) -> Json<Vec<Usage>> {
    unimplemented!();
}

//TODO: Separate usage for admin and author
#[get("/usage?<material>", format = "json")]
pub async fn get_usage_material(
    material: MaterialKey,
    auth: &Authentication,
    author: Author,
) -> Json<Vec<Usage>> {
    unimplemented!();
}

*/
