use rocket::fairing::AdHoc;
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("", |rocket| async {
        rocket.mount("/api/author", routes![get_id, authors])
    })
}

use keter_media_model::userinfo::*;
use rocket::serde::json::Json;

use crate::{auth::Unauthenticated, utility::*};

#[get("/<id>", format = "json")]
pub async fn get_id(id: UserKey, user: Unauthenticated) -> ResultNotFound<Json<AuthorInfo>, Option<String>> {
    ok_json_or_not_found(
        user.privelegies().get_author_id(id).await,
        not_found_error_string
    )
}

#[get("/", format = "json")]
pub async fn authors() -> Json<Vec<AuthorInfo>> {
    unimplemented!()
}
