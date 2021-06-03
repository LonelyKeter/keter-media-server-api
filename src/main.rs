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
        .attach(media::stage())
        .attach(authors::stage())
}
