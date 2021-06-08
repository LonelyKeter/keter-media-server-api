use rocket::fairing::AdHoc;
pub fn stage() -> AdHoc {
  AdHoc::on_ignite("", |rocket| async {
    rocket.mount("/authors", routes![
      authors
    ])
  })
}


use rocket::serde::json::Json;
use keter_media_model::{
  userinfo::*
};

#[get("/", format = "json")]
pub async fn authors() -> Json<Vec<AuthorInfo>> {
  unimplemented!()
}
