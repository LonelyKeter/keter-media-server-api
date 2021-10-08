use keter_media_db::db::model::MediaSearchKey;
use rocket::fairing::AdHoc;
use rocket::form::{self, Form, FromFormField, ValueField};
use rocket::fs::TempFile;
use rocket::serde::json::Json;
use rocket::State;

use keter_media_model::{media::*, usage::*, userinfo::*};

use crate::{auth::*, store::MaterialStore, utility::*};

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
                use_material_id,
                post_material,
                delete_material_author,
                get_reviews,
                post_review,
                get_usages,
                get_usages_material
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
pub async fn get(user: Unauthenticated) -> JsonResponce<Vec<MediaInfo>, ()> {
    JsonResponce::db_get_many(user.privelegies().get_media_many().await)
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
pub async fn get_media_id(id: MediaKey, user: Unauthenticated) -> JsonResponce<MediaInfo, ()> {
    JsonResponce::db_get_opt(user.privelegies().get_media_id(id).await)
}

#[get("/?<author_id>", format = "json")]
pub async fn get_media_author_id(
    author_id: UserKey,
    user: Unauthenticated,
) -> JsonResponce<Vec<MediaInfo>, ()> {
    JsonResponce::db_get_many(user.privelegies().get_media_author_id(author_id).await)
}

#[get("/<id>/reviews", format = "json")]
pub async fn get_reviews(id: MediaKey, user: Unauthenticated) -> JsonResponce<Vec<UserReview>, ()> {
    JsonResponce::db_get_many(user.privelegies().get_reviews(MediaSearchKey::Id(id)).await)
}

#[post("/<id>/reviews", format = "json", data = "<review>")]
pub async fn post_review(
    id: MediaKey,
    review: Json<Review>,
    user: Registered,
) -> JsonResponce<(), ()> {
    JsonResponce::db_get(
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
) -> JsonResponce<Vec<MaterialInfo>, ()> {
    JsonResponce::db_get_many(user.privelegies().get_materials(media_id).await)
}

#[get("/materials/<id>", format = "json", rank = 2)]
pub async fn get_material_id(
    id: MaterialKey,
    user: Unauthenticated,
) -> JsonResponce<MaterialInfo, ()> {
    JsonResponce::db_get_opt(user.privelegies().get_material_id(id).await)
}

#[get("/materials/<id>?used", format = "json", rank = 1)]
pub async fn get_material_id_used(
    id: MaterialKey,
    _auth: &Authentication,
    user: Registered,
) -> JsonResponce<bool, ()> {
    JsonResponce::db_get(user.privelegies().is_material_used(id).await)
}

#[post("/materials/<id>/usages", format = "json")]
pub async fn use_material_id(
    id: MaterialKey,
    auth: &Authentication,
    user: Registered,
) -> JsonResponce<(), ()> {
    JsonResponce::db_get(user.privelegies().use_material(id).await)
}

#[derive(FromForm)]
pub struct AddMaterial<'v> {
    file: TempFile<'v>,
}

#[derive(FromFormField)]
pub enum QualityParam {
    #[field(value = "very low")]
    #[field(value = "verylow")]
    VeryLow,
    Low,
    Medium,
    High,
    #[field(value = "very high")]
    #[field(value = "veryhigh")]
    VeryHigh,
}

impl Into<Quality> for QualityParam {
    fn into(self) -> Quality {
        match self {
            Self::VeryLow => Quality::VeryLow,
            Self::Low => Quality::Low,
            Self::Medium => Quality::Medium,
            Self::High => Quality::High,
            Self::VeryHigh => Quality::VeryHigh,
        }
    }
}

#[post(
    "/materials?<media>&<quality>&<format>&<license>",
    format = "multipart",
    data = "<add_material>"
)]
pub async fn post_material(
    media: MediaKey,
    quality: QualityParam,
    format: String,
    license: LicenseKey,
    add_material: Form<AddMaterial<'_>>,
    store: &State<MaterialStore>,
    _auth: &Authentication,
    author: Author,
) -> JsonResponce<(), String> {
    let material = add_material.into_inner();

    let material_id = match author
        .privelegies()
        .insert_material(
            media,
            LicenseSearchKey::Id(license),
            &format,
            quality.into(),
        )
        .await
    {
        Ok(val) => val,
        Err(err) => return JsonResponce::err(err.into()),
    };

    match store.save_material(material_id, material.file).await {
        Ok(_) => JsonResponce::success(JsonSuccess::Created(())),
        Err(err) => JsonResponce::err(JsonError::Inner(format!("{:?}", err))),
    }
}

#[delete("/materials/<id>", rank = 2)]
pub async fn delete_material_author(id: MaterialKey, auth: &Authentication, user: Author) -> JsonResponce<(), ()> {
    match user.privelegies().delete_material(id).await {
        Ok(_) => success(JsonSuccess::NoContent(())),
        Err(error) => err(error.into())
    }
}

#[delete("/materials/<id>", rank = 1)]
pub async fn delete_material_admin(id: MaterialKey, auth: &Authentication, admin: Admin) {
    unimplemented!();
}

#[get("/<id>/usages", format = "json")]
pub async fn get_usages(id: MediaKey, user: Unauthenticated) -> JsonResponce<Vec<UserUsage>, ()> {
    JsonResponce::db_get_many(user.privelegies().get_media_usages(id).await)
}

#[get("/material/<id>/usages", format = "json")]
pub async fn get_usages_material(id: MaterialKey, user: Unauthenticated) -> JsonResponce<Vec<UserUsage>, ()> {
    JsonResponce::db_get_many(user.privelegies().get_media_usages(id).await)
}