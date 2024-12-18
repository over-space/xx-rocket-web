use common::common::ResponseBody;
use rocket::{get, serde::json::Json, State};
use sea_orm::DatabaseConnection;

use crate::{employee::EmployeeEntity, employee_dao};

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

#[get("/get?<id>")]
pub async fn get_employee(db: &State<DatabaseConnection>, id: u32) -> ResponseBody<EmployeeEntity> {
    // let employee = Some(EmployeeEntity {
    //     id: 1,
    //     name: "John Doe".to_string(),
    //     age: 30,
    //     email: "john.doe@example.com".to_string(),
    // });
    // let body = ResponseBody::success(employee);

    let emp = employee_dao::get_employee(db, id).await;
    match emp {
        Ok(Some(employee)) => ResponseBody::success(Some(employee)),
        Ok(None) => ResponseBody::default(),
        Err(msg) => ResponseBody::fail(msg.to_string()),
    }
}

#[get("/list?<page>&<size>")]
pub async fn list_employee(
    db: &State<DatabaseConnection>,
    page: u64,
    size: u64,
) -> ResponseBody<Vec<EmployeeEntity>> {
    // let employee1 = EmployeeEntity {
    //     id: 1,
    //     name: "John Doe".to_string(),
    //     age: 30,
    //     email: "john.doe@example.com".to_string(),
    // };
    // let employee2 = EmployeeEntity {
    //     id: 2,
    //     name: "Jerry".to_string(),
    //     age: 21,
    //     email: "john.doe@example.com".to_string(),
    // };
    // let list = Some(vec![employee1, employee2]);
    // let body = ResponseBody::success(list);
    // Json(body)

    let emps = employee_dao::list_employee(db, page, size).await;
    match emps {
        Ok(list) => ResponseBody::new_with_vec(list),
        Err(msg) => ResponseBody::fail(msg.to_string()),
    }
}
