mod unauthenticated;
mod user;
mod user_id;

pub use unauthenticated::*;
pub use user::*;
pub use user_id::*;

use keter_media_db::auth::{Privelegies, roles};
use keter_media_db::auth as db_auth;

use rocket::{
    request::{Request, FromRequest, Outcome},
    outcome::try_outcome,
    http::Status
};

fn get_authorizator<'r>(request: &'r Request<'_>) -> Option<&'r db_auth::Authorizator> {
    request.rocket().state::<db_auth::Authorizator>()
}

#[derive(Debug)]
pub enum AccessError {
    NoPermitions,
    NoAuthToken,
    InvalidAuthToken
}