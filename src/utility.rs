use std::fmt::Debug;

use rocket::{
    serde::json::Json,
    response::{Responder}
};

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