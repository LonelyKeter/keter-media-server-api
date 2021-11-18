use super::*;

pub struct Admin {
  priveleges: Priveleges<roles::Admin>
}

impl Admin {
  fn new(priveleges: Priveleges<roles::Admin>) -> Self {
      Self { priveleges }
  }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
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