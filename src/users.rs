use crate::{auth::*, utility::*};
use keter_media_model::usage::*;
use rocket::fairing::AdHoc;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("USERS", |rocket| async {
        rocket.mount(
            "/api/users",
            routes![
                get,
                get_self,
                get_self_priveleges,
                get_usages,
                get_self_usages
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
