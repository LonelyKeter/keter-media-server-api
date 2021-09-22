use rocket::serde::json::Json;
use rocket::{fairing::AdHoc, response::content::Custom};

use keter_media_db::db::model::*;
use keter_media_model::{media::*, usage::*, userinfo::*};

use crate::{auth::*, utility::*};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("USAGE", |rocket| async {
        rocket.mount("/api/usage", routes![get_license_id, get_license_title])
    })
}

#[get("/licenses/<id>", format = "json", rank = 2)]
pub async fn get_license_id(id: LicenseKey, user: Unauthenticated) -> JsonApiResponce<License, ()> {
    JsonApiResponce::get_opt(
        user.privelegies()
            .get_license(LicenseSearchKey::Id(id))
            .await,
    )
}

#[get("/licenses/<title>", format = "json", rank = 3)]
pub async fn get_license_title(
    title: String,
    user: Unauthenticated,
) -> JsonApiResponce<License, ()> {
    JsonApiResponce::get_opt(
        user.privelegies()
            .get_license(LicenseSearchKey::Title(title))
            .await,
    )
}
