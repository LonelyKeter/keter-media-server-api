use std::ops::Deref;

use keter_media_model::{media::MaterialKey, userinfo::{UserKey}};
pub use keter_media_auth::TokenSource;

pub struct AuthTokenSource(pub TokenSource<UserKey>); 
impl Deref for AuthTokenSource {
    type Target = TokenSource<UserKey>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct DownloadTokenSource(pub TokenSource<MaterialKey>);
impl Deref for DownloadTokenSource {
    type Target = TokenSource<UserKey>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type AuthToken = String;
pub type MaterialDownloadToken = String;

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
      Err(err) => return Outcome::Forward(()) 
    };

    let token_source = match request.rocket().state::<AuthTokenSource>() {
      Some(src) => src,
      None => return Outcome::Forward(()) 
    };

    let user_key = match token_source.verify_token(token) {
      Ok(val) => val,
      Err(_) => return Outcome::Forward(()) 
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