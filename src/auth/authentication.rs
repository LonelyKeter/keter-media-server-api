use keter_media_model::userinfo::{UserKey};
use keter_media_auth::*;

pub type TokenSoure = TokenSource<UserKey>; 

use super::*;

pub struct Authentication {
  user_key: UserKey
}

impl Authentication {
  fn new(user_key: UserKey) -> Self {
      Self { user_key }
  }

  pub fn user_key(&self) -> UserKey { self.user_key }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r Authentication {
  type Error = AccessError;

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let token = match get_token_from_auth_header(request) {
      Ok(val) => val,
      Err(err) => return Outcome::Failure((Status::BadRequest, err))
    };

    let token_source = match request.rocket().state::<TokenSoure>() {
      Some(src) => src,
      None => return Outcome::Failure((Status::ServiceUnavailable, AccessError::InvalidAuthToken))
    };

    let user_key = match token_source.verify_token_str(token) {
      Ok(val) => val,
      Err(_) => return Outcome::Failure((Status::Unauthorized, AccessError::InvalidAuthToken))
    };

    let authentication = request.local_cache(|| Authentication::new(user_key));
    Outcome::Success(authentication)
  }  
}

fn get_token_from_cookies<'r>(request: &'r Request<'_>) -> Result<&'r str, AccessError> {
  if let Some(cookie) = request.cookies().get(JWT_COOCKIE_NAME) {
    Ok(cookie.value())
  } else {
    Err(AccessError::NoAuthToken)
  }
}

fn get_token_from_auth_header<'r>(request: &'r Request<'_>) -> Result<&'r str, AccessError> {
  if let Some(header) = request.headers().get_one("Authorization") {
    if header.starts_with("Bearer ") {
      Ok(header.trim_start_matches("Bearer "))
    } else {
      Err(AccessError::InvalidAuthScheme)
    }
  } else {
    Err(AccessError::NoAuthToken)
  }
}