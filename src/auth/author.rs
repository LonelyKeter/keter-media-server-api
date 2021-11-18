use super::*;

pub struct Author {
  priveleges: Priveleges<roles::Author>
}

impl Author {
  fn new(priveleges: Priveleges<roles::Author>) -> Self {
      Self { priveleges }
  }

  pub fn priveleges(&self) -> &Priveleges<roles::Author> {
    &self.priveleges
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
      
      match auhtorizator.author_priveleges(authentication.user_key()).await {
          Ok(priveleges) => Outcome::Success(Self::new(priveleges)),
          Err(_) => Outcome::Failure((Status::Forbidden, AccessError::NoPermitions))
      }
  }  
}