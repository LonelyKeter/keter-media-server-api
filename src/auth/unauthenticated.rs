use std::ops::Deref;

use keter_media_db::client::Client;

use super::*;

pub struct Unauthenticated<'a> {
  client: &'a Client<roles::Unauthenticated>
}

impl<'a> Unauthenticated<'a> {
  fn new<'b: 'a>(client: &'b Client<roles::Unauthenticated>) -> Self {
      Self { client }
  }
}

impl Deref for Unauthenticated<'_> {
    type Target = Client<roles::Unauthenticated>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Unauthenticated<'r> {
  type Error = AccessError;

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
      let authorizator = match get_authorizator(request) {
          Some(val) => val,
          None => return Outcome::Forward(())
      };
      
      match authorizator.unauthenticated_priveleges().await {
          Ok(priveleges) => Outcome::Success(Self::new(priveleges)),
          Err(_) => Outcome::Failure((Status::Forbidden, AccessError::NoPermitions))
      }
  }  
}