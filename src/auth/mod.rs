mod unauthenticated;
mod user;
mod authentication;
mod author;
mod moderator;
mod admin;

mod requests;
mod responders;

pub use unauthenticated::*;
pub use user::*;
pub use author::*;
pub use authentication::*;
pub use moderator::*;
pub use admin::*;

pub use requests::stage;

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