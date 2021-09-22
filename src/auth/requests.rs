use keter_media_db::auth::{AuthenticationError, Authenticator};
use keter_media_model::userinfo::{self, RegisterData};
use rocket::{fairing::AdHoc, response::status, serde::json::Json, State};

use keter_media_model::userinfo::*;

use crate::{auth::*, state, utility::*};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("AUTH", |rocket| async {
        rocket.mount(
            "/api/auth",
            routes![login, register, get_self, get_privelegies],
        )
    })
}

#[get("/self")]
pub async fn get_self(
    auth: &Authentication,
    user: Registered,
) -> JsonApiResponce<UserInfo, ()> {
    JsonApiResponce::get_opt(user.privelegies().get_info().await)
}

#[get("/privelegies")]
pub async fn get_privelegies(
    auth: &Authentication,
    user: Registered,
) -> JsonApiResponce<UserPriveleges, ()> {
    JsonApiResponce::get_opt(user.privelegies().get_privelegies().await)
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

#[post("/login", format = "json", data = "<login_data>")]
pub async fn login(
    login_data: Json<userinfo::LoginData>,
    token_source: &State<authentication::TokenSoure>,
    authenticator: &State<state::Authenticator>,
) -> Result<status::Accepted<String>, status::BadRequest<()>> {
    match token_source
        .create_token_async(LoginData(login_data.0), authenticator)
        .await
    {
        Ok(token) => Ok(status::Accepted(Some(token))),
        Err(err) => {
            //eprintln!("{:?}", err);
            Err(status::BadRequest(None))
        }
    }
}

#[post("/register", format = "json", data = "<register_data>")]
pub async fn register(
    register_data: Json<RegisterData>,
    auth: &State<state::Authenticator>,
) -> Result<status::Accepted<()>, status::BadRequest<()>> {
    match auth.register(register_data.0).await {
        Ok(_) => Ok(status::Accepted(Some(()))),
        Err(_) => Err(status::BadRequest(Some(()))),
    }
}
