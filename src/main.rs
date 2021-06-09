#![allow(dead_code)]
#![allow(unused_variables)]

mod media;
mod authors;

mod auth;

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

mod type_prelude {
    pub use rocket::{
        serde:: {
            Serialize, Deserialize
        } 
    };
}

struct Init {
    authorizator: Authorizator
}

impl Init {
    async fn init() -> Result<Self, ()> {
        use rocket::tokio::spawn;
        let authorizator = spawn(create_authorizator())
            .await
            .map_err(|_| ())?
            .map_err(|_| ())?;

        Ok(Self {
            authorizator
        })
    }
}

#[rocket::main]
async fn main() {
    let init = Init::init().await.unwrap();
    

    rocket::build()
        .manage(init.authorizator)
        .attach(media::stage())
        .attach(authors::stage())
        .launch()
        .await;
}

use auth::Authentication;
use keter_media_db::auth::Authorizator;
async fn create_authorizator() -> Result<Authorizator, ClientError> {
    use keter_media_db::auth::ModelDBClients;
    let auth_client = create_auth_db().auth().await?;
    let media_db = create_media_db();
    
    let authorizator = Authorizator::new(
        auth_client, 
        ModelDBClients::from_model_db(&media_db).await?);

    Ok(authorizator)
}

use keter_media_db::client::ClientError;
use keter_media_db::db::auth::AuthDB;
fn create_auth_db() -> AuthDB {
    unimplemented!()
}



fn create_media_db() -> keter_media_db::db::ModelDB {
    unimplemented!()
}