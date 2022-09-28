use actix_web::{
    // error::ResponseError,
    get,
    // http::{header::ContentType, StatusCode},
    // post, put,
    // web::Data,
    web::Json,
    web::Path,
    // HttpResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserIdentifier {
    user_id: String,
}

#[get("/users")]
pub async fn get_users() -> Json<String> {
    return Json("userlist!!".to_string());
}

#[get("/user/{user_id}")]
pub async fn get_user(user_identifier: Path<UserIdentifier>) -> Json<String> {
    return Json(user_identifier.into_inner().user_id.to_string());
}
