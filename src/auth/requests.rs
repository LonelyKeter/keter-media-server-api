use keter_media_db::auth::{Authenticator, AuthenticationError};
use keter_media_model::userinfo::{self, RegisterData};
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

use super::responders::BearerAuth;

pub fn stage() -> AdHoc {
  AdHoc::on_ignite("AUTH", |rocket| async {
    rocket.mount("/auth", routes![
        login,
        register
    ])
  })
}

#[get("/self", format = "json")]
pub async fn get_self(auth: &Authentication, user: Registered) 
    -> ResultNotFound<Json<UserInfo>, ()> {
    ok_json_or_not_found(
        user.privelegies().get_info().await, 
        ())
}

#[get("/privelegies", format = "json")]
pub async fn get_privelegies(auth: &Authentication, user: Registered) 
    -> ResultNotFound<Json<UserPrivelegies>, ()> {
    ok_json_or_not_found(
        user.privelegies().get_privelegies().await,
        ())
}

struct LoginData(userinfo::LoginData);

#[rocket::async_trait]
impl keter_media_auth::LoginDataAsync for LoginData {
    type Claim = UserKey;
    type Context = Authenticator;
    type Err = AuthenticationError;

    async fn to_claim(self, context: &Self::Context) -> Result<Self::Claim, Self::Err> {
        let user_id = context.authenticate(self.0).await?;
        Ok(user_id)
    }
}

#[post("/login", format = "json", data="<login_data>")]
pub async fn login(login_data: Json<userinfo::LoginData>, 
    token_source: &State<authentication::TokenSoure>, authenticator: &State<state::Authenticator>) 
    -> Result<status::Accepted<BearerAuth<()>>, status::BadRequest<()>> {
    if let Ok(token) = token_source.create_token_async(LoginData(login_data.0), authenticator).await {
        Ok(status::Accepted(Some(BearerAuth::new(token, ()))))
    } else {
        Err(status::BadRequest(Some(())))
    }
}

#[post("/register", format = "json", data="<register_data>")]
pub async fn register(register_data: Json<RegisterData>, auth: &State<state::Authenticator>) 
    -> Result<status::Accepted<()>, status::BadRequest<()>> {
    match auth.register(register_data.0).await {
        Ok(_) => Ok(status::Accepted(Some(()))),
        Err(_) => Err(status::BadRequest(Some(())))
    }
}  