use super::*;

pub struct Author {
  privelegies: Privelegies<roles::Author>
}

impl Author {
  fn new(privelegies: Privelegies<roles::Author>) -> Self {
      Self { privelegies }
  }

  pub fn privelegies(&self) -> Privelegies<roles::Author> {
    self.privelegies
  }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Author {
  type Error = AccessError;

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
      let authentication = try_outcome!(request.guard::<&Authentication>().await);

      let auhtorizator = match get_authorizator(request) {
          Some(val) => val,
          None => return Outcome::Forward(())
      };
      
      match auhtorizator.author_privelegies(authentication.user_id()).await {
          Ok(privelegies) => Outcome::Success(Self::new(privelegies)),
          Err(_) => Outcome::Failure((Status::Forbidden, AccessError::NoPermitions))
      }
  }  
}