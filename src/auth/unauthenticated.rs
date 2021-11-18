use super::*;

pub struct Unauthenticated {
  priveleges: Priveleges<roles::Unauthenticated>
}

impl Unauthenticated {
  fn new(priveleges: Priveleges<roles::Unauthenticated>) -> Self {
      Self { priveleges }
  }

  pub fn priveleges(&self) -> &Priveleges<roles::Unauthenticated> {
    &self.priveleges
  }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Unauthenticated {
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