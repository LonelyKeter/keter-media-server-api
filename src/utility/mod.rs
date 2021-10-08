use std::fmt::Debug;

use keter_media_db::client::ClientError;
pub use serde::Serialize;

pub use rocket::response::status;

mod json;
pub use json::*;

//mod plain;
//pub use plain::*;

pub type ResultNotFound<T, E> = Result<T, status::NotFound<E>>;
pub type ResultCustom<T, E> = Result<T, status::Custom<E>>;
pub type ResultUnauthorized<T, E> = Result<T, status::Unauthorized<E>>;