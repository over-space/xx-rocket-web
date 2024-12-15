use common::controller::index;
use employee::controller::{get_employee, list_employee, none};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    config::init::dotenv();

    rocket::build()
        .mount("/", routes![index])
        .mount("/employee", routes![none, get_employee, list_employee])
}
