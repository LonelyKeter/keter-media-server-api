use keter_media_db::auth::AuthenticationInfo;
use rocket::{
    State,
    fairing::AdHoc,
    form::{self, FromFormField, ValueField},
    serde::{json::Json},
    response::{status},
    request::{Request}
};

use keter_media_model::{
  userinfo::*
};

use crate::{
    auth::*,
    utility::*,
    state,
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
#[post("/login", format = "json", data="<auth_info>")]
pub async fn login(auth_info: Json<AuthenticationInfo>, 
    token_source: &State<authentication::TokenSoure>, authenticator: &State<state::Authenticator>) 
    -> Result<status::Accepted<()>, status::BadRequest<()>> {
    if let Ok(user_id) = authenticator.authenticate(auth_info.0).await {
        if let Ok(token) = token_source.create_token(user_id) {
            
        }
    }
}

pub async fn register(register_data: RegisterData, auth: &State<state::Authenticator>) 
    -> Result<status::Accepted<UserInfo>, status::BadRequest<()>> {
    todo!("Register logic")
}  