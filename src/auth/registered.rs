use super::*;

pub struct Registered {
  priveleges: Priveleges<roles::Registered>
}

impl Registered {
  fn new(priveleges: Priveleges<roles::Registered>) -> Self {
      Self { priveleges }
  }

  pub fn priveleges(&self) -> &Priveleges<roles::Registered> {
    &self.priveleges
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
      
      match auhtorizator.registered_priveleges(authentication.user_key()).await {
          Ok(priveleges) => Outcome::Success(Self::new(priveleges)),
          Err(_) => Outcome::Failure((Status::Forbidden, AccessError::NoPermitions))
      }
  }  
}