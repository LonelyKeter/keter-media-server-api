use super::*;

pub struct Admin {
  privelegies: Privelegies<roles::Admin>
}

impl Admin {
  fn new(privelegies: Privelegies<roles::Admin>) -> Self {
      Self { privelegies }
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
      
      match auhtorizator.admin_privelegies(authentication.user_key()).await {
          Ok(privelegies) => Outcome::Success(Self::new(privelegies)),
          Err(_) => Outcome::Failure((Status::Forbidden, AccessError::NoPermitions))
      }
  }  
}