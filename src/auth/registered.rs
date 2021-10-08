use super::*;

pub struct Registered {
  privelegies: Privelegies<roles::Registered>
}

impl Registered {
  fn new(privelegies: Privelegies<roles::Registered>) -> Self {
      Self { privelegies }
  }

  pub fn privelegies(&self) -> &Privelegies<roles::Registered> {
    &self.privelegies
  }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Registered {
  type Error = AccessError;

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
      let authentication = try_outcome!(request.guard::<&Authentication>().await);

      let auhtorizator = match get_authorizator(request) {
          Some(val) => val,
          None => return Outcome::Failure((Status::InternalServerError, AccessError::NoPermitions))
      };
      
      match auhtorizator.registered_privelegies(authentication.user_key()).await {
          Ok(privelegies) => Outcome::Success(Self::new(privelegies)),
          Err(_) => Outcome::Failure((Status::Forbidden, AccessError::NoPermitions))
      }
  }  
}