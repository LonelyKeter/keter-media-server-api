use std::fmt::Debug;

use keter_media_db::client::ClientError;
pub use serde::Serialize;

use rocket::{http::Status, response::{Responder}, serde::json::{self, Json}};

pub use rocket::response::status;

pub fn ok_json_or_not_found<'r, 'o: 'r, T: rocket::serde::Serialize, E, R: Responder<'r, 'o>>(
    db_query_result: Result<Option<T>, E>,
    not_found_respond: impl Fn(Option<E>) -> R) 
    -> ResultNotFound<Json<T>, R> {
    match db_query_result {
        Ok(Some(value)) => Ok(Json(value)),
        Ok(None) => Err(status::NotFound(not_found_respond(None))),
        Err(e) => Err(status::NotFound(not_found_respond(Some(e)))),
    }  
}

pub fn ok_json_vec_or_not_found<'r, 'o: 'r, T: rocket::serde::Serialize, E, R: Responder<'r, 'o>>(
  db_query_result: Result<Vec<T>, E>,
  not_found_respond: impl Fn(Option<E>) -> R) 
  -> ResultNotFound<Json<Vec<T>>, R> {
  match db_query_result {
      Ok(values) => Ok(Json(values)),
      Err(e) => Err(status::NotFound(not_found_respond(Some(e)))),
  }  
}

pub fn not_found_error_string(err: Option<impl Debug>) -> Option<String> {
  err.map(|e| format!("{:?}", e))
}

pub type ResultNotFound<T, E> = Result<T, status::NotFound<E>>;
pub type ResultCustom<T, E> = Result<T, status::Custom<E>>;
pub type ResultUnauthorized<T, E> = Result<T, status::Unauthorized<E>>;
pub struct JsonApiResponce<T: Serialize, E: Serialize>(Result<Json<T>, JsonApiError<E>>);

impl<T: Serialize, E: Serialize> JsonApiResponce<T, E> {
    pub fn ok(result: T) -> Self {
        Self(Ok(Json(result)))
    }

    pub fn err(err: JsonApiError<E>) -> Self {
        Self(Err(err))
    }  
}

impl<'r, 'o: 'r, T: Serialize, E: Serialize> Responder<'r, 'o> for JsonApiResponce<T, E> {
    #[inline(always)]
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        self.0.respond_to(request)
    }
}

impl<T: Serialize> JsonApiResponce<T, ()> {
    pub fn get_many(db_query_result: Result<Vec<T>, ClientError>) -> JsonApiResponce<Vec<T>, ()> {
        match db_query_result {
            Ok(val) => JsonApiResponce::ok(val),
            Err(cl_err) => JsonApiResponce::err(cl_err.into())
        }
    }

    pub fn get_opt(db_query_result: Result<Option<T>, ClientError>) -> JsonApiResponce<T, ()> {
        match db_query_result {
            Ok(Some(val)) => JsonApiResponce::ok(val),
            Ok(None) => JsonApiResponce::err(JsonApiError::NotFound),
            Err(cl_err) => JsonApiResponce::err(cl_err.into())
        }
    }

    pub fn get(db_query_result: Result<T, ClientError>) -> JsonApiResponce<T, ()> {
        match db_query_result {
            Ok(val) => JsonApiResponce::ok(val),
            Err(cl_err) => JsonApiResponce::err(cl_err.into())
        }
    }
}



#[derive(Debug, Serialize)]
pub enum JsonApiError<E: Serialize> {
    Db { message: String },
    NotFound,
    Inner { error: E }
}

impl<'r, 'o: 'r, E: Serialize> Responder<'r, 'o> for JsonApiError<E> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        match self {
            Self::Db {message: _} => status::Custom(Status::InternalServerError, Json(self)).respond_to(request),
            Self::Inner {error: _} => status::Custom(Status::InternalServerError, Json(self)).respond_to(request),
            Self::NotFound => status::Custom(Status::NotFound, Json(self)).respond_to(request),
        }
    }
}

impl<E: Serialize> From<ClientError> for JsonApiError<E> {
    fn from(err: ClientError) -> Self {
        match err {
            ClientError::Parse(e) => Self::Db {
                message: format!("{}", e),
            },
            ClientError::Postgres(e) => Self::Db {
                message: format!("{}", e),
            },
            _ => Self::Db {
                message: "".to_string(),
            }
        }
    }
}