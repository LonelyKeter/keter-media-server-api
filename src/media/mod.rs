mod types;
mod requests; 

use requests::*;

use rocket::fairing::AdHoc;
pub fn stage() -> AdHoc {
  AdHoc::on_ignite("MEDIA", |rocket| async {
    rocket.mount("/media", routes![
      media_base
    ])
  })
}