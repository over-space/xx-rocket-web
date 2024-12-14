use common::common::ResponseBody;
use rocket::{get, serde::json::Json};

use crate::employee::EmployeeEntity;


#[get("/none?<age>")]
pub fn none(age: u8) -> Json<ResponseBody<EmployeeEntity>> {
    // if age < 0u8 {
    //     let body = ResponseBody::fail("年龄不合理".to_owned());
    //     Json(body)
    // } else if age >= 121u8 {
    //     let body = ResponseBody::fail("年龄不合理".to_owned());
    //     Json(body)
    // } else {
    //     let body = ResponseBody::default();
    //     Json(body)
    // }
    let body = match age {
        0..=120 => ResponseBody::default(),
        _ => ResponseBody::fail("年龄不合理".to_owned()),
    };
    Json(body)
}

#[get("/get")]
pub fn get_employee() -> Json<ResponseBody<EmployeeEntity>> {
    let employee = Some(EmployeeEntity {
        id: 1,
        name: "John Doe".to_string(),
        age: 30,
        email: "john.doe@example.com".to_string(),
    });
    let body = ResponseBody::success(employee);
    Json(body)
}

#[get("/list")]
pub fn list_employee() -> Json<ResponseBody<Vec<EmployeeEntity>>> {
    let employee1 = EmployeeEntity {
        id: 1,
        name: "John Doe".to_string(),
        age: 30,
        email: "john.doe@example.com".to_string(),
    };
    let employee2 = EmployeeEntity {
        id: 2,
        name: "Jerry".to_string(),
        age: 21,
        email: "john.doe@example.com".to_string(),
    };
    let list = Some(vec![employee1, employee2]);
    let body = ResponseBody::success(list);
    Json(body)
}
