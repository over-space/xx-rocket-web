use rocket::{get, serde::json::Json};

use crate::common::ResponseBody;

#[get("/")]
pub fn index() -> Json<ResponseBody<String>> {
    let body = ResponseBody::default();
    Json(body)
}