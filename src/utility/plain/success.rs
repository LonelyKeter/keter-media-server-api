use rocket::response::{Responder, status};
use crate::utility::*;

pub enum PlainSuccess<T: Debug> {
    
}

impl<'r, 'o: 'r, T: Debug> Responder<'r, 'o> for PlainSuccess<T> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        
    }
}