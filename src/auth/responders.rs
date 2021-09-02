use rocket::{
  request::Request,
  response::{self, Responder},
  http::{ Cookie, hyper::header::AUTHORIZATION, Header },
};

use super::*;

pub struct BearerAuth<R> {
  token: String,
  inner: R
}

impl<R> BearerAuth<R> {
  pub fn new (token: String, inner: R) -> Self {
    Self { token, inner }   
  }
}

impl<'r, R: Responder<'r, 'static>> Responder<'r, 'static> for BearerAuth<R> {
  fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
    let mut response = self.inner.respond_to(request)?;

    let mut value = "BEARER ".to_owned();
    value.push_str(&self.token);
    response.set_header(Header::new(AUTHORIZATION.as_str(), value));

    Ok(response)
  }    
}

pub struct HttpOnlyJWT<R> {
  token: String,
  inner: R
}

impl<R> HttpOnlyJWT<R> {
  pub fn new (token: String, inner: R) -> Self {
    Self { token, inner }   
  }
}

impl<'r, R: Responder<'r, 'static>> Responder<'r, 'static> for HttpOnlyJWT<R> {
  fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
    let response = self.inner.respond_to(request)?;

    let jar = request.cookies();

    let cookie = Cookie::build(JWT_COOCKIE_NAME, self.token)
      .path("/")
      .http_only(true)
      .finish();

    jar.add(cookie);

    Ok(response)
  }    
}
