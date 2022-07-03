#[macro_use] extern crate rocket;

mod user_interface;
use crate::user_interface::api::rest::resource_permission::routes::resource_access_index;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/resource", routes![resource_access_index])
}
