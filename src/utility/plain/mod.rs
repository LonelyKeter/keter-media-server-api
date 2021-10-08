mod success;
pub use success::*;

mod error;
pub use error::*;

use rocket::response::{Responder, status};
use crate::utility::*;

pub struct PlainResponce<T: Debug, E: Debug>(Result<PlainSuccess<T>, PlainError<E>>);

impl<T: Debug, E: Debug> PlainResponce<T, E> {
    pub fn ok(result: T) -> Self {
        Self(Ok(result))
    }

    pub fn err(err: PlainError<E>) -> Self {
        Self(Err(err))
    }
}

impl<'r, 'o: 'r, T: Debug, E: Debug> Responder<'r, 'o> for PlainResponce<T, E> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {

    }
}