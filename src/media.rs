use keter_media_db::db::model::MediaSearchKey;
use rocket::fairing::AdHoc;
use rocket::form::{self, Form, FromFormField, ValueField};
use rocket::fs::TempFile;
use rocket::serde::json::Json;
use rocket::State;

use crate::{auth::*, store::MaterialStore, utility::*};
use keter_media_model::{media::*, usage::*, userinfo::*, *};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("MEDIA", |rocket| async {
        rocket.mount(
            "/api/media",
            routes![
                get,
                get_with_options,
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

#[get("/")]
pub async fn get(user: Unauthenticated<'_>) -> JsonResponce<Vec<MediaInfo>, ()> {
    JsonResponce::db_get_many(user.get_media_many().await)
}

#[derive(FromFormField)]
pub enum MediaKindParam {
    #[field(value = "a")]
    #[field(value = "audio")]
    Audio,
    #[field(value = "v")]
    #[field(value = "video")]
    Video,
    #[field(value = "i")]
    #[field(value = "image")]
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

#[derive(FromFormField)]
pub enum FilterOrderingParam {
    #[field(value = "asc")]
    #[field(value = "ascending")]
    Ascending,
    #[field(value = "desc")]
    #[field(value = "descending")]
    Descending,
}

impl From<FilterOrderingParam> for FilterOrdering {
    fn from(other: FilterOrderingParam) -> FilterOrdering {
        match other {
            FilterOrderingParam::Ascending => FilterOrdering::Ascending,
            FilterOrderingParam::Descending => FilterOrdering::Descending,
        }
    }
}

#[get("/?<title>&<kinds>&<min_rating>&<max_rating>&<min_use_count>&<max_use_count>&<rating_ordering>&<use_count_ordering>", format = "json", rank=2)]
pub async fn get_with_options(
    title: Option<String>,
    kinds: Option<Vec<MediaKindParam>>,
    min_rating: Option<i64>,
    max_rating: Option<i64>,
    min_use_count: Option<i64>,
    max_use_count: Option<i64>,
    rating_ordering: Option<FilterOrderingParam>,
    use_count_ordering: Option<FilterOrderingParam>,
    user: Unauthenticated<'_>,
) -> JsonResponce<Vec<MediaInfo>, ()> {
    let kinds = kinds.map(|v| v.into_iter().map(Into::into).collect());

    let rating_ordering = rating_ordering.map(Into::<FilterOrdering>::into);
    let use_count_ordering = use_count_ordering.map(Into::<FilterOrdering>::into);

    let options = parse_media_filter_options(
        title,
        kinds,
        min_rating,
        max_rating,
        min_use_count,
        max_use_count,
        rating_ordering,
        use_count_ordering,
    );

    JsonResponce::db_get_many(user.get_media_many_with_options(&options).await)
}

fn parse_media_filter_options(
    title: Option<String>,
    kinds: Option<Vec<MediaKind>>,
    min_rating: Option<i64>,
    max_rating: Option<i64>,
    min_use_count: Option<i64>,
    max_use_count: Option<i64>,
    rating_ordering: Option<FilterOrdering>,
    use_count_ordering: Option<FilterOrdering>,
) -> MediaFilterOptions {
    let popularity = parse_range_filter(min_rating, max_rating, rating_ordering);
    let times_used = parse_range_filter(min_use_count, max_use_count, use_count_ordering);

    MediaFilterOptions {
        title,
        kinds,
        popularity,
        times_used,
    }
}

fn parse_range_filter(
    min: Option<i64>,
    max: Option<i64>,
    ordering: Option<FilterOrdering>,
) -> Option<RangeFilter> {
    if min.is_some() || max.is_some() || ordering.is_some() {
        Some(RangeFilter {
            ordering,
            limits: Limits { min, max },
        })
    } else {
        None
    }
}

#[get("/<id>", format = "json")]
pub async fn get_media_id(id: MediaKey, user: Unauthenticated<'_>) -> JsonResponce<MediaInfo, ()> {
    JsonResponce::db_get_opt(user.get_media_id(id).await)
}

#[get("/?<author_id>", format = "json", rank = 3)]
pub async fn get_media_author_id(
    author_id: UserKey,
    user: Unauthenticated<'_>,
) -> JsonResponce<Vec<MediaInfo>, ()> {
    JsonResponce::db_get_many(user.get_media_author_id(author_id).await)
}

#[get("/<id>/usages", format = "json")]
pub async fn get_usages(
    id: MediaKey,
    user: Unauthenticated<'_>,
) -> JsonResponce<Vec<UserUsage>, ()> {
    JsonResponce::db_get_many(user.get_media_usages(id).await)
}

#[get("/reviews/<review_id>", format = "json", rank = 3)]
pub async fn get_review(
    review_id: ReviewKey,
    user: Unauthenticated<'_>,
) -> JsonResponce<UserReview, ()> {
    JsonResponce::db_get_opt(user.get_review(review_id).await)
}

#[get("/<media_id>/reviews", format = "json")]
pub async fn get_reviews(
    media_id: MediaKey,
    user: Unauthenticated<'_>,
) -> JsonResponce<Vec<UserReview>, ()> {
    JsonResponce::db_get_many(user.get_reviews(&MediaSearchKey::Id(media_id)).await)
}

#[post("/<media_id>/reviews", format = "json", data = "<review>")]
pub async fn post_review(
    media_id: MediaKey,
    review: Json<Review>,
    auth: &Authentication,
    user: Registered<'_>,
) -> JsonResponce<(), ()> {
    JsonResponce::db_get(
        user.post_review(auth.user_key(), &MediaSearchKey::Id(media_id), &review.0)
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
    author: Author<'_>,
) -> JsonResponce<MediaKey, ()> {
    JsonResponce::db_insert(author.create_media(auth.user_key(), &reg_media).await)
}

#[post("/", format = "json", rank = 4)]
pub async fn post_media_reject() -> JsonError<()> {
    JsonError::Unauthorized(())
}

#[get("/<media_id>/materials", format = "json", rank = 1)]
pub async fn get_materials_auth(
    media_id: MediaKey,
    auth: &Authentication,
    user: Unauthenticated<'_>,
) -> JsonResponce<Vec<MaterialInfo>, ()> {
    JsonResponce::db_get_many(user.get_materials(media_id, Some(auth.user_key())).await)
}

#[get("/<media_id>/materials", format = "json", rank = 2)]
pub async fn get_materials(
    media_id: MediaKey,
    user: Unauthenticated<'_>,
) -> JsonResponce<Vec<MaterialInfo>, ()> {
    JsonResponce::db_get_many(user.get_materials(media_id, None).await)
}

#[get("/materials/<material_id>", format = "json", rank = 3)]
pub async fn get_material_id_auth(
    material_id: MaterialKey,
    auth: &Authentication,
    user: Unauthenticated<'_>,
) -> JsonResponce<MaterialInfo, ()> {
    JsonResponce::db_get_opt(
        user.get_material_id(material_id, Some(auth.user_key()))
            .await,
    )
}

#[get("/materials/<material_id>", format = "json", rank = 4)]
pub async fn get_material_id(
    material_id: MaterialKey,
    user: Unauthenticated<'_>,
) -> JsonResponce<MaterialInfo, ()> {
    JsonResponce::db_get_opt(user.get_material_id(material_id, None).await)
}

#[get("/materials/<material_id>/usages?<user_id>", format = "json", rank = 1)]
pub async fn get_material_id_user_usage(
    material_id: MaterialKey,
    user_id: UserKey,
    user: Unauthenticated<'_>,
) -> JsonResponce<Usage, ()> {
    JsonResponce::db_get_opt(user.get_material_usage_user_id(material_id, user_id).await)
}

#[get("/material/<id>/usages", format = "json")]
pub async fn get_usages_material(
    id: MaterialKey,
    user: Unauthenticated<'_>,
) -> JsonResponce<Vec<UserUsage>, ()> {
    JsonResponce::db_get_many(user.get_media_usages(id).await)
}

#[post("/materials/<material_id>/usages", format = "json")]
pub async fn post_material_id_usage(
    material_id: MaterialKey,
    auth: &Authentication,
    user: Registered<'_>,
) -> JsonResponce<(), ()> {
    JsonResponce::db_get(
        user.priveleges()
            .create_material_usage(auth.user_key(), material_id)
            .await,
    )
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
    type Context = keter_media_db::client::Client<keter_media_db::auth::roles::Unauthenticated>;
    type Err = JsonError<()>;

    async fn to_claim(self, client: &Self::Context) -> Result<Self::Claim, Self::Err> {
        match client
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
    user: Unauthenticated<'_>,
    token_source: &State<DownloadTokenSource>,
) -> JsonResponce<String, ()> {
    match token_source
        .create_token_async(
            DonwloadMaterial {
                material_id,
                user_id: auth.user_key(),
            },
            &user,
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
    user: Unauthenticated<'_>,
) -> Result<status::Accepted<FileResponce>, JsonError<()>> {
    let material_id = token_source
        .verify_token(&token)
        .map_err(|_| JsonError::BadRequest(()))?;

    let file = store
        .get_material(material_id)
        .await
        .map_err(|_| JsonError::InternalServerError(()))?;

    let file_name = user
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
    user: Registered<'_>,
) -> JsonResponce<(), ()> {
    JsonResponce::db_put(
        user.insert_material_rating(material_id, auth.user_key(), &user_rating)
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
    auth: &Authentication,
    author: Author<'_>,
) -> JsonResponce<MaterialKey, String> {
    let material = add_material.into_inner();

    let material_id = match author
        .insert_material(
            auth.user_key(),
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
pub async fn delete_material_admin(id: MaterialKey, auth: &Authentication, admin: Admin<'_>) {
    unimplemented!();
}

#[delete("/materials/<id>", rank = 2)]
pub async fn delete_material_author(
    id: MaterialKey,
    auth: &Authentication,
    user: Author<'_>,
) -> JsonResponce<(), ()> {
    match user.delete_material(auth.user_key(), id).await {
        Ok(_) => success(JsonSuccess::NoContent(())),
        Err(error) => err(error.into()),
    }
}

#[delete("/materials/<id>", rank = 3)]
pub async fn delete_material_reject(id: MaterialKey) -> JsonError<()> {
    JsonError::Unauthorized(())
}
