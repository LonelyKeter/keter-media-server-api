use super::*;

pub struct Unauthenticated {
  privelegies: Privelegies<roles::Unauthenticated>
}

impl Unauthenticated {
  fn new(privelegies: Privelegies<roles::Unauthenticated>) -> Self {
      Self { privelegies }
  }

  pub fn privelegies(&self) -> &Privelegies<roles::Unauthenticated> {
    &self.privelegies
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
      
      match authorizator.unauthenticated_privelegies().await {
          Ok(privelegies) => Outcome::Success(Self::new(privelegies)),
          Err(_) => Outcome::Failure((Status::Forbidden, AccessError::NoPermitions))
      }
  }  
}