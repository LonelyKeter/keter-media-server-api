use crate::utility::*;
use rocket::{http::Status, response::Responder, serde::json::Json};

macro_rules! impl_json_error {
    ($($code:ident),*) => {
        #[derive(Debug, Serialize)]
        #[serde(tag = "errorKind", content = "payload")]
        pub enum JsonError<T: Serialize>{
            Db { message: String },
            $($code(T),)*
        }


        impl<'r, 'o: 'r, T: Serialize> Responder<'r, 'o> for JsonError<T> {
            fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
                match self {
                    Self::Db { ref message } => status::Custom(Status::InternalServerError, Json(self)).respond_to(request),
                    $(JsonError::$code(_) => status::Custom(Status::$code, Json(self)).respond_to(request),)*
                }
            }
        }
    };
}

impl_json_error!(
    InternalServerError,
    NotFound,
    BadRequest,
    Unauthorized,
    Forbidden
);

impl<E: Serialize> From<ClientError> for JsonError<E> {
    fn from(err: ClientError) -> Self {
        match err {
            ClientError::Parse(e) => Self::Db {
                message: format!("{:#?}", e),
            },
            ClientError::Postgres(e) => Self::Db {
                message: if let Some(db_err) = e.as_db_error() {
                    format!("{:#?}", db_err)
                } else {
                    String::from("")
                },
            },
            _ => Self::Db {
                message: "".to_string(),
            },
        }
    }
}

impl<T: Serialize, E: Serialize> From<Result<JsonSuccess<T>, JsonError<E>>> for JsonResponce<T, E> {
    fn from(result: Result<JsonSuccess<T>, JsonError<E>>) -> Self {
        Self(result)
    }
}
