mod types;
mod requests; 

use requests::*;

use rocket::fairing::AdHoc;
pub fn stage() -> AdHoc {
  AdHoc::on_ignite("", |rocket| async {
    rocket.mount("", routes![
      
    ])
  })
}