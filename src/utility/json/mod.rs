mod success;
pub use success::*;

mod error;
pub use error::*;

use crate::utility::*;
use rocket::response::Responder;

#[inline(always)]
pub fn success<T: Serialize, E: Serialize>(success: JsonSuccess<T>) -> JsonResponce<T, E> {
    JsonResponce::success(success)
}

#[inline(always)]
pub fn err<T: Serialize, E: Serialize>(err: JsonError<E>) -> JsonResponce<T, E> {
    JsonResponce::err(err)
}


pub struct JsonResponce<T: Serialize, E: Serialize>(Result<JsonSuccess<T>, JsonError<E>>);

impl<T: Serialize, E: Serialize> JsonResponce<T, E> {
    pub fn success(success: JsonSuccess<T>) -> Self {
        Self(Ok(success))
    }

    pub fn err(err: JsonError<E>) -> Self {
        Self(Err(err))
    }
}

impl<'r, 'o: 'r, T: Serialize, E: Serialize> Responder<'r, 'o> for JsonResponce<T, E> {
    #[inline(always)]
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        self.0.respond_to(request)
    }
}

impl<T: Serialize> JsonResponce<T, ()> {
    pub fn db_get_many(
        db_query_result: Result<Vec<T>, ClientError>,
    ) -> JsonResponce<Vec<T>, ()> {
        match db_query_result {
            Ok(val) => JsonResponce::success(JsonSuccess::Ok(val)),
            Err(cl_err) => JsonResponce::err(cl_err.into()),
        }
    }

    pub fn db_get_opt(db_query_result: Result<Option<T>, ClientError>) -> JsonResponce<T, ()> {
        match db_query_result {
            Ok(Some(val)) => JsonResponce::success(JsonSuccess::Ok(val)),
            Ok(None) => JsonResponce::err(JsonError::NotFound(())),
            Err(cl_err) => JsonResponce::err(cl_err.into()),
        }
    }

    pub fn db_get(db_query_result: Result<T, ClientError>) -> JsonResponce<T, ()> {
        match db_query_result {
            Ok(val) => JsonResponce::success(JsonSuccess::Ok(val)),
            Err(cl_err) => JsonResponce::err(cl_err.into()),
        }
    }
}