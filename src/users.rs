use crate::{auth::*, utility::*};
use keter_media_model::{usage::*};
use rocket::fairing::AdHoc;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("USERS", |rocket| async {
        rocket.mount("/api/users", routes![
            get,
            get_self,
            get_self_privelegies,
            get_usages,
            get_self_usages])
    })
}

use keter_media_model::userinfo::*;

#[get("/<id>", format = "json")]
pub async fn get(id: UserKey, user: Unauthenticated) -> JsonResponce<UserInfo, ()> {
    JsonResponce::db_get_opt(user.privelegies().get_user_info(id).await)
}

#[get("/self")]
pub async fn get_self(
    _auth: &Authentication,
    user: Registered,
) -> JsonResponce<UserInfo, ()> {
    JsonResponce::db_get_opt(user.privelegies().get_info().await)
}

#[get("/self/privelegies")]
pub async fn get_self_privelegies(
    _auth: &Authentication,
    user: Registered,
) -> JsonResponce<UserPriveleges, ()> {
    JsonResponce::db_get_opt(user.privelegies().get_privelegies().await)
}

#[get("/<id>/usages")]
pub async fn get_usages(id: UserKey, user: Unauthenticated) -> JsonResponce<Vec<Usage>, ()> {
    JsonResponce::db_get_many(user.privelegies().get_usages(id).await)
}

#[get("/self/usages", rank = 2)]
pub async fn get_self_usages(_auth: &Authentication, user: Unauthenticated) -> JsonResponce<Vec<Usage>, ()> {
    JsonResponce::db_get_many(user.privelegies().get_usages(_auth.user_key()).await)
}