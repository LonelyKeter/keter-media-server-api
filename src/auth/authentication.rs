use keter_media_model::userinfo::{UserKey};
use keter_media_auth::*;

pub type TokenSoure = TokenSource<UserKey>; 

use super::*;

pub struct Authentication {
  user_key: UserKey
}

impl Authentication {
  fn new(user_key: UserKey) -> Self {
      Self { user_key: user_key }
  }

  pub fn user_key(&self) -> UserKey { self.user_key }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r Authentication {
  type Error = AccessError;

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let header = match request.headers().get_one("Authorization") {
      Some(val) => val,
      None => return Outcome::Failure((Status::Unauthorized, AccessError::NoAuthToken))
    };

    if !header.starts_with("Bearer ") {
      return Outcome::Failure((Status::Unauthorized, AccessError::NoAuthToken))
    }

    //TODO: normal token retrieval from header
    let token = header.trim_start_matches("Bearer ");

    let token_source = match request.rocket().state::<TokenSoure>() {
      Some(src) => src,
      None => return Outcome::Failure((Status::Unauthorized, AccessError::InvalidAuthToken))
    };

    let user_key = match token_source.verify_token_str(token) {
      Ok(val) => val,
      Err(_) => return Outcome::Failure((Status::Unauthorized, AccessError::InvalidAuthToken))
    };

    let authentication = request.local_cache(|| Authentication::new(user_key));
    Outcome::Success(authentication)
  }  
}