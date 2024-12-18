use rocket::{response::Responder, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    code: i32,
    message: String,
    data: Option<T>,
}

impl<T> ResponseBody<T> {
    pub fn success(data: Option<T>) -> Self {
        ResponseBody::new(0, "success".to_owned(), data)
    }

    pub fn new_with_vec(data: Vec<T>) -> ResponseBody<Vec<T>> {
        ResponseBody {
            code: 0,
            message: "success".to_owned(),
            data: Some(data),
        }
    }

    pub fn new_with_none_data(code: i32, message: String) -> Self {
        ResponseBody::new(code, message, None)
    }

    pub fn new(code: i32, message: String, data: Option<T>) -> Self {
        ResponseBody {
            code,
            message,
            data,
        }
    }

    pub fn fail(message: String) -> Self {
        ResponseBody::new_with_none_data(1000, message)
    }
}

impl<T> Default for ResponseBody<T> {
    fn default() -> Self {
        ResponseBody::success(None)
    }
}

#[rocket::async_trait]
impl<'r, T: serde::Serialize> Responder<'r, 'static> for ResponseBody<T> {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        Json(self).respond_to(req)
    }
}