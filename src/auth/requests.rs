use rocket::{
    State,
    fairing::AdHoc,
    form::{self, FromFormField, ValueField},
    serde::{json::Json},
    response::{status},
};

use keter_media_model::{
  userinfo::*
};

use crate::{
    auth::*,
    utility::*
};

pub fn stage() -> AdHoc {
  AdHoc::on_ignite("AUTH", |rocket| async {
    rocket.mount("/auth", routes![
        get_base,
        get_self,
        get_privelegies
    ])
  })
}

#[get("/", format = "json")]
pub async fn get_base(auth: &Authentication) 
    -> Result<status::Accepted<()>, status::Unauthorized<()>> {
    
}

#[get("/self", format = "json")]
pub async fn get_self(auth: &Authentication, user: User) 
    -> ResultNotFound<Json<UserInfo>, ()> {
    ok_json_or_not_found(
        user.privelegies().get_info().await, 
        ())
}

#[get("/privelegies", format = "json")]
pub async fn get_privelegies(auth: &Authentication, user: User) 
    -> ResultNotFound<Json<UserPrivelegies>, ()> {
    ok_json_or_not_found(
        user.privelegies().get_privelegies().await,
        ())
}

#[post("/login", format = "json", data="<login_data>")]
pub async fn login(login_data: LoginData, token_source: &State<authentication::TokenSoure>) 
    -> Result<status::Accepted<()>, status::BadRequest<()>> {
    todo!("Login logic")
}

pub async fn register(register_data: RegisterData, auth: &State<Authenticator>) 
    -> Result<status::Accepted<UserInfo>, status::BadRequest<()>> {
    todo!("Register logic")
}  