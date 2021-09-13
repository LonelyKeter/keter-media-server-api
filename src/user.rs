use rocket::fairing::AdHoc;
pub fn stage() -> AdHoc {
  AdHoc::on_ignite("", |rocket| async {
    rocket.mount("/api/user", routes![
      authors
    ])
  })
}


use rocket::serde::json::Json;
use keter_media_model::{
  userinfo::*
};

#[get("/<id>", format = "json")]
pub async fn get(id: UserKey) -> Json<Vec<UserInfo>> {
  unimplemented!()
}
