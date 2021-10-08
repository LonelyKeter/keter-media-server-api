use std::fmt::Display;

use rocket::{Response, http::Status, response::Responder};
use crate::utility::*;

#[derive(Debug)]
pub enum PlainError<E: Display> {
    Unauthorized(E)
}

impl<'r, 'o: 'r, E: Display> Responder<'r, 'o> for PlainError<E> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        
    }
}
