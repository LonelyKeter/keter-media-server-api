use rocket::{
    serde::json::Json,
    response::{Responder, status}
};

pub fn ok_json_or_not_found<'r, 'o: 'r, T: rocket::serde::Serialize, E, R: Responder<'r, 'o>>(
    db_query_result: Result<Option<T>, E>,
    not_found_respond: R) 
    -> ResultNotFound<Json<T>, R> {
    if let Ok(Some(value)) = db_query_result {
        Ok(Json(value))
    } else {
        return Err(status::NotFound(not_found_respond));
    }  
}

pub type ResultNotFound<T, E> = Result<T, status::NotFound<E>>;