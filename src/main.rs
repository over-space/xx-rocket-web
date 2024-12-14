use employee::employee::{none, get_employee, list_employee};


#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/employee", routes![none, get_employee, list_employee])
}
