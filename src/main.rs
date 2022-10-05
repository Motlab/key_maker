#[macro_use] extern crate rocket;

mod user_interface;
use crate::user_interface::api::rest::resource_permission::routes::resource_access_index;

use log4rs;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();

    rocket::build()
        .mount("/", routes![index])
        .mount("/resource", routes![resource_access_index])
}
