#![allow(dead_code)]
#![allow(unused_variables)]

mod licenses;
mod media;
mod users;

mod auth;

mod utility;

mod store;

#[macro_use]
pub extern crate rocket;

use pretty_unwrap::*;

mod req_prelude {
    pub use rocket::{self, http::RawStr, serde::json::Json};
}

pub mod state {
    pub use keter_media_db::auth::Authenticator;
}

struct Init {
    authorizator: Authorizator,
    authenticator: Authenticator,
    token_source: auth::AuthTokenSource,
    store_token_source: auth::DownloadTokenSource,
    material_store: store::MaterialStore,
}

impl Init {
    async fn init() -> Result<Self, InitError> {
        use crate::auth::{AuthTokenSource, DownloadTokenSource, TokenSource};
        use keter_media_model::media::MediaKey;
        use keter_media_model::userinfo::UserKey;

        let authorizator = create_authorizator();
        let authenticator = create_authenticator();

        let token_source = AuthTokenSource(TokenSource::<UserKey>::from_secret(
            b"Very very secret secret",
        ));
        let store_token_source = DownloadTokenSource(TokenSource::<MediaKey>::from_secret(
            b"Very very secret secret",
        ));

        let material_store = store::MaterialStore::init("\\store");

        Ok(Self {
            authorizator: authorizator.await.map_err(InitError::Client)?,
            authenticator: authenticator.await.map_err(InitError::Client)?,
            material_store: material_store.await.map_err(InitError::MaterialStore)?,
            token_source,
            store_token_source,
        })
    }
}

#[derive(Debug)]
enum InitError {
    Client(ClientError),
    MaterialStore(store::Error),
}

#[rocket::main]
async fn main() {
    let init = Init::init().await.unwrap_pretty();

    build_app(init).launch().await.unwrap_pretty();
}

use rocket::{Build, Rocket};
fn build_app(init: Init) -> Rocket<Build> {
    rocket::build()
        .manage(init.authorizator)
        .manage(init.authenticator)
        .manage(init.material_store)
        .manage(init.token_source)
        .manage(init.store_token_source)
        .attach(media::stage())
        .attach(users::stage())
        .attach(licenses::stage())
        .attach(auth::stage())
}

use keter_media_db::auth::{Authenticator, Authorizator};
async fn create_authorizator() -> Result<Authorizator, ClientError> {
    use keter_media_db::auth::ModelDBClients;
    let auth_client = create_auth_db().auth().await?;
    let model_db = create_model_db();

    let authorizator =
        Authorizator::new(auth_client, ModelDBClients::from_model_db(&model_db).await?);

    Ok(authorizator)
}

async fn create_authenticator() -> Result<Authenticator, ClientError> {
    let auth_client = create_auth_db().auth().await?;
    let authenticator = Authenticator::new(auth_client);

    Ok(authenticator)
}

use keter_media_db::{
    client::ClientError,
    db::{AuthDB, ModelDB},
};

fn create_auth_db() -> AuthDB {
    AuthDB::default()
}

fn create_model_db() -> keter_media_db::db::ModelDB {
    ModelDB::default()
}

#[cfg(test)]
mod test {
    use super::*;
    use rocket::local::asynchronous::Client;
    use rocket::{
        http::{Header, Status},
        Rocket,
    };

    async fn client_untracked<P: rocket::Phase>(rocket: Rocket<P>) -> Client {
        Client::untracked(rocket)
            .await
            .expect("Valid rocket instance")
    }

    #[async_test]
    async fn login() {
        use keter_media_model::userinfo::LoginData;

        let rocket = build_app(Init::init().await.unwrap_pretty());
        let client = client_untracked(rocket).await;

        let login_data = LoginData {
            email: String::from("firstuser@mail.com"),
            password: String::from("First user"),
        };

        let responce = client
            .post("/api/auth/login")
            .json(&login_data)
            .dispatch()
            .await;

        assert_eq!(responce.status(), Status::Accepted);

        let token = responce.into_string().await.unwrap();

        let mut request = client.get("/api/auth/self");
        let header = Header::new("Authorization", format!("Bearer {}", token));
        request.add_header(header);

        let responce = request.dispatch().await;

        assert_eq!(responce.status(), Status::Ok);
    }
}
