use keter_media_db::auth::{AuthenticationError, Authenticator};
use keter_media_model::userinfo::{self, RegisterData};
use rocket::{fairing::AdHoc, response::status, serde::json::Json, State};

use keter_media_model::userinfo::*;

use crate::{auth::*, state, utility::*};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("AUTH", |rocket| async {
        rocket.mount(
            "/api/auth",
            routes![login, register],
        )
    })
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
    token_source: &State<authentication::AuthTokenSource>,
    authenticator: &State<state::Authenticator>,
) -> JsonResponce<AuthToken, String> {
    match token_source
        .create_token_async(LoginData(login_data.0), authenticator)
        .await
    {
        Ok(token) => success(JsonSuccess::Accepted(token)),
        Err(_) => {
            //eprintln!("{:?}", err);
            err(JsonError::BadRequest(String::from("Invalid token")))
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
