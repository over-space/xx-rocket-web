use common::controller::index;
use employee::controller::{get_employee, list_employee, none};
use sea_orm::DatabaseConnection;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // 获取mysql数据库连接
    let db: DatabaseConnection = config::init::init_mysq_pool().await;

    // 启动 Rocket
    let _ = rocket::build()
        .mount("/", routes![index])
        .mount("/employee", routes![get_employee, list_employee, none])
        .manage(db)
        .launch()
        .await?;

    Ok(())
}
