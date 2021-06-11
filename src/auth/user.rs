use super::*;

pub struct User {
  privelegies: Privelegies<roles::User>
}

impl User {
  fn new(privelegies: Privelegies<roles::User>) -> Self {
      Self { privelegies }
  }

  pub fn privelegies(&self) -> &Privelegies<roles::User> {
    &self.privelegies
  }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
  type Error = AccessError;

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
      let authentication = try_outcome!(request.guard::<&Authentication>().await);

      let auhtorizator = match get_authorizator(request) {
          Some(val) => val,
          None => return Outcome::Forward(())
      };
      
      match auhtorizator.user_privelegies(authentication.user_key()).await {
          Ok(privelegies) => Outcome::Success(Self::new(privelegies)),
          Err(_) => Outcome::Failure((Status::Forbidden, AccessError::NoPermitions))
      }
  }  
}