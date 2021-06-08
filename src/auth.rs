use keter_media_auth::*;
use keter_media_db::auth::{Privelegies, roles::*};

use rocket::{
    request::{Request, FromRequest, Outcome},
    responce::Status
};

pub struct Unauthenticated {
    privelegies: Privelegies<Unauthenticated>
}

impl FromRequest for Unauthenticated {
    type Error = Error;

    fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        
    }  
}

#[derive(debug)]
Error {

}