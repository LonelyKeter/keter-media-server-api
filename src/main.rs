#![allow(dead_code)]
#![allow(unused_variables)]

mod media;
mod authors;

mod auth;

mod utility;

#[macro_use] pub extern crate rocket;

mod req_prelude {
    pub use rocket::{
            self,
            http::
            {
                RawStr
            },
            serde:: {
                json::Json
            } 
        };
}

pub mod state {
    pub use keter_media_db::auth::Authenticator;
}

struct Init {
    authorizator: Authorizator,
    authenticator: Authenticator,
    token_source: auth::TokenSoure
}

impl Init {
    async fn init() -> Result<Self, ()> {
        use rocket::tokio::spawn;
        let authorizator = spawn(create_authorizator())
            .await
            .map_err(|_| ())?
            .map_err(|_| ())?;

        let authenticator = create_authenticator()
            .await
            .map_err(|_| ())?;

        let token_source = crate::auth::TokenSoure::from_secret(b"Very very secret secret");

        Ok(Self {
            authorizator,
            authenticator,
            token_source
        })
    }
}

#[rocket::main]
async fn main() {
    let init = Init::init().await.unwrap();
    

    rocket::build()
        .manage(init.authorizator)
        .manage(init.authenticator)
        .manage(init.token_source)
        .attach(media::stage())
        .attach(authors::stage())
        .attach(auth::stage())
        .launch()
        .await
        .unwrap();
}

use auth::Authentication;
use keter_media_db::auth::Authenticator;
use keter_media_db::auth::Authorizator;
async fn create_authorizator() -> Result<Authorizator, ClientError> {
    use keter_media_db::auth::ModelDBClients;
    let auth_client = create_auth_db().auth().await?;
    let model_db = create_model_db();
    
    let authorizator = Authorizator::new(
        auth_client, 
        ModelDBClients::from_model_db(&model_db).await?);

    Ok(authorizator)
}

async fn create_authenticator() -> Result<Authenticator, ClientError> {
    let auth_client = create_auth_db().auth().await?;
    let authenticator = Authenticator::new(auth_client);

    Ok(authenticator)
}

use keter_media_db::client::ClientError;
use keter_media_db::db::ModelDB;
use keter_media_db::db::auth::AuthDB;
fn create_auth_db() -> AuthDB {
    AuthDB::default()
}



fn create_model_db() -> keter_media_db::db::ModelDB {
    ModelDB::default()
}