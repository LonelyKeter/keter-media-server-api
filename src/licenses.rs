use keter_media_model::usage::*;
use rocket::fairing::AdHoc;

use crate::{auth::*, utility::*};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("LICENSES", |rocket| async {
        rocket.mount(
            "/api/licenses",
            routes![get_many, get_with_id, get_with_title],
        )
    })
}

#[get("/")]
pub async fn get_many(user: Unauthenticated<'_>) -> JsonResponce<Vec<License>, ()> {
    JsonResponce::db_get_many(user.get_licenses_many().await)
}

#[get("/<id>", format = "json", rank = 2)]
pub async fn get_with_id(id: LicenseKey, user: Unauthenticated<'_>) -> JsonResponce<License, ()> {
    JsonResponce::db_get_opt(user.get_license(&LicenseSearchKey::Id(id)).await)
}

#[get("/<title>", format = "json", rank = 3)]
pub async fn get_with_title(title: String, user: Unauthenticated<'_>) -> JsonResponce<License, ()> {
    JsonResponce::db_get_opt(user.get_license(&LicenseSearchKey::Title(title)).await)
}
