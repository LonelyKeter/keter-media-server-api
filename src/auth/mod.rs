mod unauthenticated;
mod registered;
mod authentication;
mod author;
mod moderator;
mod admin;

mod requests;
mod responders;

pub use unauthenticated::*;
pub use registered::*;
pub use author::*;
pub use authentication::*;
pub use moderator::*;
pub use admin::*;

pub use requests::stage;

use keter_media_db::auth::{roles};
use keter_media_db::auth as db_auth;

use rocket::{
    request::{Request, FromRequest, Outcome},
    outcome::try_outcome,
    http::Status
};

static JWT_COOCKIE_NAME: &str = "JWTauth";

fn get_authorizator<'r>(request: &'r Request<'_>) -> Option<&'r db_auth::Authorizator> {
    request.rocket().state::<db_auth::Authorizator>()
}

#[derive(Debug)]
pub enum AccessError {
    NoPermitions,
    NoAuthToken,
    InvalidAuthToken,
    InvalidAuthScheme
}