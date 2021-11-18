use keter_media_db::auth::Priveleges;
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
                post_media,
                post_media_reject,
                get_materials,
                get_materials_auth,
                get_material_id,
                get_material_id_auth,
                get_material_id_user_usage,
                get_material_download_token,
                get_material_download_token_reject,
                download_material,
                post_material_id_usage,
                post_material,
                post_material_reject,
                delete_material_author,
                delete_material_reject,
                get_reviews,
                post_review,
                post_review_reject,
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
    JsonResponce::db_get_many(user.priveleges().get_media_many().await)
}

#[get("/?<title>&<kind>&<author>&<rating>", format = "json")]
pub async fn get_base(
    title: Option<String>,
    kind: Option<MediaKindParam>,
    author: Option<AuthorParam>,
    rating: Option<Rating>,
) -> Json<Vec<MediaInfo>> {
    unimplemented!()
}

#[get("/<id>", format = "json")]
pub async fn get_media_id(id: MediaKey, user: Unauthenticated) -> JsonResponce<MediaInfo, ()> {
    JsonResponce::db_get_opt(user.priveleges().get_media_id(id).await)
}

#[get("/?<author_id>", format = "json")]
pub async fn get_media_author_id(
    author_id: UserKey,
    user: Unauthenticated,
) -> JsonResponce<Vec<MediaInfo>, ()> {
    JsonResponce::db_get_many(user.priveleges().get_media_author_id(author_id).await)
}

#[get("/<id>/usages", format = "json")]
pub async fn get_usages(id: MediaKey, user: Unauthenticated) -> JsonResponce<Vec<UserUsage>, ()> {
    JsonResponce::db_get_many(user.priveleges().get_media_usages(id).await)
}

#[get("/reviews/<review_id>", format = "json", rank = 3)]
pub async fn get_review(review_id: ReviewKey, user: Unauthenticated) -> JsonResponce<UserReview, ()> {
    JsonResponce::db_get_opt(user.priveleges().get_review(review_id).await)
}

#[get("/<media_id>/reviews", format = "json")]
pub async fn get_reviews(
    media_id: MediaKey,
    user: Unauthenticated,
) -> JsonResponce<Vec<UserReview>, ()> {
    JsonResponce::db_get_many(
        user.priveleges()
            .get_reviews(MediaSearchKey::Id(media_id))
            .await,
    )
}

#[post("/<media_id>/reviews", format = "json", data = "<review>")]
pub async fn post_review(
    media_id: MediaKey,
    review: Json<Review>,
    _auth: &Authentication,
    user: Registered,
) -> JsonResponce<(), ()> {
    JsonResponce::db_get(
        user.priveleges()
            .post_review(&MediaSearchKey::Id(media_id), &review.0)
            .await,
    )
}

#[post("/<media_id>/reviews", format = "json", data = "<review>", rank = 3)]
pub async fn post_review_reject(media_id: MediaKey, review: Json<Review>) -> JsonError<()> {
    JsonError::Unauthorized(())
}

#[post("/", format = "json", data = "<reg_media>")]
pub async fn post_media(
    reg_media: Json<RegisterMedia>,
    auth: &Authentication,
    author: Author,
) -> JsonResponce<MediaKey, ()> {
    JsonResponce::db_insert(author.priveleges().create_media(&reg_media).await)
}

#[post("/", format = "json", rank = 4)]
pub async fn post_media_reject() -> JsonError<()> {
    JsonError::Unauthorized(())
}

#[get("/<media_id>/materials", format = "json", rank = 1)]
pub async fn get_materials_auth(
    media_id: MediaKey,
    auth: &Authentication,
    user: Unauthenticated,
) -> JsonResponce<Vec<MaterialInfo>, ()> {
    JsonResponce::db_get_many(
        user.priveleges()
            .get_materials(media_id, Some(auth.user_key()))
            .await,
    )
}

#[get("/<media_id>/materials", format = "json", rank = 2)]
pub async fn get_materials(
    media_id: MediaKey,
    user: Unauthenticated,
) -> JsonResponce<Vec<MaterialInfo>, ()> {
    JsonResponce::db_get_many(user.priveleges().get_materials(media_id, None).await)
}

#[get("/materials/<material_id>", format = "json", rank = 3)]
pub async fn get_material_id_auth(
    material_id: MaterialKey,
    auth: &Authentication,
    user: Unauthenticated,
) -> JsonResponce<MaterialInfo, ()> {
    JsonResponce::db_get_opt(
        user.priveleges()
            .get_material_id(material_id, Some(auth.user_key()))
            .await,
    )
}

#[get("/materials/<material_id>", format = "json", rank = 4)]
pub async fn get_material_id(
    material_id: MaterialKey,
    user: Unauthenticated,
) -> JsonResponce<MaterialInfo, ()> {
    JsonResponce::db_get_opt(user.priveleges().get_material_id(material_id, None).await)
}

#[get("/materials/<material_id>/usages?<user_id>", format = "json", rank = 1)]
pub async fn get_material_id_user_usage(
    material_id: MaterialKey,
    user_id: UserKey,
    user: Unauthenticated,
) -> JsonResponce<Usage, ()> {
    JsonResponce::db_get_opt(
        user.priveleges()
            .get_material_usage_user_id(material_id, user_id)
            .await,
    )
}

#[get("/material/<id>/usages", format = "json")]
pub async fn get_usages_material(
    id: MaterialKey,
    user: Unauthenticated,
) -> JsonResponce<Vec<UserUsage>, ()> {
    JsonResponce::db_get_many(user.priveleges().get_media_usages(id).await)
}

#[post("/materials/<material_id>/usages", format = "json")]
pub async fn post_material_id_usage(
    material_id: MaterialKey,
    auth: &Authentication,
    user: Registered,
) -> JsonResponce<(), ()> {
    JsonResponce::db_get(user.priveleges().create_material_usage(material_id).await)
}

#[post("/materials/<material_id>/usages", format = "json")]
pub async fn use_material_reject(material_id: MaterialKey) -> JsonError<()> {
    JsonError::Unauthorized(())
}

use rocket::serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DonwloadMaterial {
    material_id: MaterialKey,
    user_id: UserKey,
}

#[rocket::async_trait]
impl keter_media_auth::LoginDataAsync for DonwloadMaterial {
    type Claim = MaterialKey;
    type Context = Priveleges<keter_media_db::auth::roles::Unauthenticated>;
    type Err = JsonError<()>;

    async fn to_claim(self, priveleges: &Self::Context) -> Result<Self::Claim, Self::Err> {
        match priveleges
            .get_material_usage_user_id(self.material_id, self.user_id)
            .await
        {
            //Unwrap is valid because implementation is infallible
            Ok(Some(val)) => Ok(self.material_id),
            Ok(None) => Err(JsonError::Forbidden(())),
            Err(error) => Err(error.into()),
        }
    }
}

#[get("/materials/<material_id>/token", rank = 2)]
pub async fn get_material_download_token(
    material_id: MaterialKey,
    auth: &Authentication,
    user: Unauthenticated,
    token_source: &State<DownloadTokenSource>,
) -> JsonResponce<String, ()> {
    match token_source
        .create_token_async(
            DonwloadMaterial {
                material_id,
                user_id: auth.user_key(),
            },
            user.priveleges(),
        )
        .await
    {
        Ok(token) => success(JsonSuccess::Accepted(token)),
        Err(error) => err(error),
    }
}

#[get("/materials/<material_id>/token", rank = 3)]
pub async fn get_material_download_token_reject(material_id: MaterialKey) -> JsonError<()> {
    JsonError::Unauthorized(())
}

#[get("/materials/download?<token>")]
pub async fn download_material(
    token: String,
    token_source: &State<DownloadTokenSource>,
    store: &State<MaterialStore>,
    user: Unauthenticated,
) -> Result<status::Accepted<FileResponce>, JsonError<()>> {
    let material_id = token_source
        .verify_token(&token)
        .map_err(|_| JsonError::BadRequest(()))?;

    let file = store
        .get_material(material_id)
        .await
        .map_err(|_| JsonError::InternalServerError(()))?;

    let file_name = user
        .priveleges()
        .get_material_download_name(material_id)
        .await
        .map_err(|err| JsonError::from(err))?;

    let mut file_responce = FileResponce::new(file);

    file_responce
        .with_file_name(file_name.into())
        .extension_from_file_name();

    Ok(status::Accepted(Some(file_responce)))
}

#[put("/materials/<material_id>/rating", data = "<user_rating>")]
pub async fn put_material_rating(
    material_id: MaterialKey,
    user_rating: Json<UserRating>,
    auth: &Authentication,
    user: Registered,
) -> JsonResponce<(), ()> {
    JsonResponce::db_put(
        user.priveleges()
            .insert_material_rating(material_id, &user_rating)
            .await,
    )
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
) -> JsonResponce<MaterialKey, String> {
    let material = add_material.into_inner();

    let material_id = match author
        .priveleges()
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
        Ok(_) => JsonResponce::success(JsonSuccess::Created(material_id)),
        Err(err) => JsonResponce::err(JsonError::InternalServerError(format!("{:?}", err))),
    }
}

#[post(
    "/materials?<media>&<quality>&<format>&<license>",
    format = "multipart",
    rank = 2
)]
pub async fn post_material_reject(
    media: MediaKey,
    quality: QualityParam,
    format: String,
    license: LicenseKey,
) -> JsonError<()> {
    JsonError::Unauthorized(())
}

#[delete("/materials/<id>")]
pub async fn delete_material_admin(id: MaterialKey, auth: &Authentication, admin: Admin) {
    unimplemented!();
}

#[delete("/materials/<id>", rank = 2)]
pub async fn delete_material_author(
    id: MaterialKey,
    auth: &Authentication,
    user: Author,
) -> JsonResponce<(), ()> {
    match user.priveleges().delete_material(id).await {
        Ok(_) => success(JsonSuccess::NoContent(())),
        Err(error) => err(error.into()),
    }
}

#[delete("/materials/<id>", rank = 3)]
pub async fn delete_material_reject(id: MaterialKey) -> JsonError<()> {
    JsonError::Unauthorized(())
}
