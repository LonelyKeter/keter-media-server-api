use std::ops::Deref;

use keter_media_db::client::Client;

use super::*;

pub struct Admin<'a> {
  client: &'a Client<roles::Admin>
}

impl<'a> Admin<'a> {
  fn new<'b: 'a>(client: &'b Client<roles::Admin>) -> Self {
      Self { client }
  }
}

impl Deref for Admin<'_> {
    type Target = Client<roles::Admin>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin<'r> {
  type Error = AccessError;

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
      let authentication = try_outcome!(request.guard::<&Authentication>().await);

      let auhtorizator = match get_authorizator(request) {
          Some(val) => val,
          None => return Outcome::Forward(())
      };
      
      match auhtorizator.admin_priveleges(authentication.user_key()).await {
          Ok(priveleges) => Outcome::Success(Self::new(priveleges)),
          Err(_) => Outcome::Failure((Status::Forbidden, AccessError::NoPermitions))
      }
  }  
}