use crate::{
    auth::*,
    param_mappings::*,
    utility::*,
};
use keter_media_model::{*, usage::*};
use rocket::{fairing::AdHoc, serde::json::Json};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("USERS", |rocket| async {
        rocket.mount(
            "/api/users",
            routes![
                get,
                get_self,
                get_self_priveleges,
                get_usages,
                get_self_usages,
                get_authors
            ],
        )
    })
}

use keter_media_model::userinfo::*;

#[get("/<id>", format = "json")]
pub async fn get(id: UserKey, user: Unauthenticated<'_>) -> JsonResponce<UserInfo, ()> {
    JsonResponce::db_get_opt(user.get_user_info(id).await)
}

#[get("/self")]
pub async fn get_self(
    _auth: &Authentication,
    user: Unauthenticated<'_>,
) -> JsonResponce<UserInfo, ()> {
    JsonResponce::db_get_opt(user.get_user_info(_auth.user_key()).await)
}

#[get("/self/priveleges")]
pub async fn get_self_priveleges(
    _auth: &Authentication,
    user: Unauthenticated<'_>,
) -> JsonResponce<UserPriveleges, ()> {
    JsonResponce::db_get_opt(user.get_user_priveleges(_auth.user_key()).await)
}

#[get("/<id>/usages")]
pub async fn get_usages(id: UserKey, user: Unauthenticated<'_>) -> JsonResponce<Vec<Usage>, ()> {
    JsonResponce::db_get_many(user.get_user_usages(id).await)
}

#[get("/self/usages", rank = 2)]
pub async fn get_self_usages(
    auth: &Authentication,
    user: Unauthenticated<'_>,
) -> JsonResponce<Vec<Usage>, ()> {
    JsonResponce::db_get_many(user.get_user_usages(auth.user_key()).await)
}

#[post("/authors", data = "<contacts>")]
pub async fn create_author(
    auth: &Authentication,
    user: Registered<'_>,
    contacts: Json<AuthorContacts>,
) -> JsonResponce<(), ()> {
    unimplemented!("Create author")
}

#[get("/authors?<name>&<kinds>&<min_rating>&<max_rating>&<rating_ordering>")]
pub async fn get_authors(
    name: Option<String>,
    kinds: Option<Vec<MediaKindParam>>,
    min_rating: Option<i64>,
    max_rating: Option<i64>,
    rating_ordering: Option<FilterOrderingParam>,
    user: Unauthenticated<'_>,
) -> JsonResponce<Vec<AuthorInfo>, ()> {
    let kinds = kinds.map(|v| v.into_iter().map(Into::into).collect());

    let rating_ordering = rating_ordering.map(Into::<FilterOrdering>::into);
    let popularity = parse_range_filter(min_rating, max_rating, rating_ordering);

    let options = AuthorFilterOptions {
        name,
        kinds,
        popularity
    };

    JsonResponce::db_get_many(user.get_authors_filtered(&options).await)
}

