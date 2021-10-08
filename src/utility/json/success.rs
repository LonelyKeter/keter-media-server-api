use crate::utility::*;
use rocket::{http::Status, response::Responder, serde::json::Json};

macro_rules! impl_json_success {
    ($($code:ident),*) => {
        pub enum JsonSuccess<T: Serialize>{ 
            $($code(T),)*
        }

        
        impl<'r, 'o: 'r, T: Serialize> Responder<'r, 'o> for JsonSuccess<T> {
            fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
                match self {
                    $(JsonSuccess::$code(val) => status::Custom(Status::$code, Json(val)).respond_to(request),)*
                }
            }
        }
    };
}

impl_json_success!(Ok, Created, Accepted, NoContent);