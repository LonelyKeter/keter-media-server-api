use super::*;

pub struct User {
  privelegies: Privelegies<roles::User>
}

impl User {
  fn new(privelegies: Privelegies<roles::User>) -> Self {
      Self { privelegies }
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
      
      match auhtorizator.user_privelegies(authentication.user_id()).await {
          Ok(privelegies) => Outcome::Success(Self::new(privelegies)),
          Err(_) => Outcome::Failure((Status::Forbidden, AccessError::NoPermitions))
      }
  }  
}