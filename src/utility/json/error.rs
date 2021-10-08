use rocket::{http::Status, response::Responder, serde::json::Json};
use crate::utility::*;

#[derive(Debug, Serialize)]
#[serde(tag = "errorKind", content = "payload")]
pub enum JsonError<E: Serialize> {
    Db { message: String },
    NotFound(E),
    Inner(E),
    BadRequest(E)
}

impl<'r, 'o: 'r, E: Serialize> Responder<'r, 'o> for JsonError<E> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        match self {
            Self::Db {message: _} => status::Custom(Status::InternalServerError, Json(self)).respond_to(request),
            Self::Inner(_) => status::Custom(Status::InternalServerError, Json(self)).respond_to(request),
            Self::NotFound(_) => status::Custom(Status::NotFound, Json(self)).respond_to(request),
            Self::BadRequest(_) => status::Custom(Status::BadRequest, Json(self)).respond_to(request),
        }
    }
}

impl<E: Serialize> From<ClientError> for JsonError<E> {
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