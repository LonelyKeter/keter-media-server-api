use rocket::fairing::AdHoc;
use crate::{*, auth::Unauthenticated, utility::*};

pub fn stage() -> AdHoc {
  AdHoc::on_ignite("USERS", |rocket| async {
    rocket.mount("/api/users", routes![
      get
    ])
  })
}


use rocket::serde::json::Json;
use keter_media_model::{
  userinfo::*
};

#[get("/<id>", format = "json")]
pub async fn get(id: UserKey, user: Unauthenticated) -> ResultNotFound<Json<UserInfo>, Option<String>> {
    ok_json_or_not_found(
        user.privelegies().get_user_info(id).await,
        not_found_error_string
    )
}
