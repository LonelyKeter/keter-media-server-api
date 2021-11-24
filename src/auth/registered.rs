use std::ops::Deref;

use keter_media_db::client::Client;

use super::*;

pub struct Registered<'a> {
  client: &'a Client<roles::Registered>
}

impl<'a> Registered<'a> {
  fn new<'b: 'a>(client: &'b Client<roles::Registered>) -> Self {
      Self { client }
  }

  pub fn priveleges(&self) -> &Client<roles::Registered> {
    &self.client
  }
}

impl Deref for Registered<'_> {
    type Target = Client<roles::Registered>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Registered<'r> {
  type Error = AccessError;

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
      let authentication = try_outcome!(request.guard::<&Authentication>().await);

      let auhtorizator = match get_authorizator(request) {
          Some(val) => val,
          None => return Outcome::Failure((Status::InternalServerError, AccessError::NoPermitions))
      };
      
      match auhtorizator.registered_priveleges().await {
          Ok(priveleges) => Outcome::Success(Self::new(priveleges)),
          Err(_) => Outcome::Failure((Status::Forbidden, AccessError::NoPermitions))
      }
  }  
}