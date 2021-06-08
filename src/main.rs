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


#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(create_authorizator())
        .attach(media::stage())
        .attach(authors::stage())
}

fn create_authorizator() -> keter_media_db::auth::Authorizator {
    unimplemented!()
}

fn get_media_db() -> keter_media_db::db::ModelDB {
    unimplemented!()
}