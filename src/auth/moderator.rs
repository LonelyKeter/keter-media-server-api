use super::*;

pub struct Moderator {
  privelegies: Privelegies<roles::Moderator>
}

impl Moderator {
  fn new(privelegies: Privelegies<roles::Moderator>) -> Self {
      Self { privelegies }
  }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Moderator {
  type Error = AccessError;

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
      let authentication = try_outcome!(request.guard::<&Authentication>().await);

      let auhtorizator = match get_authorizator(request) {
          Some(val) => val,
          None => return Outcome::Forward(())
      };
      
      match auhtorizator.moderator_privelegies(authentication.user_key()).await {
          Ok(privelegies) => Outcome::Success(Self::new(privelegies)),
          Err(_) => Outcome::Failure((Status::Forbidden, AccessError::NoPermitions))
      }
  }  
}