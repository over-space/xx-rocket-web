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
