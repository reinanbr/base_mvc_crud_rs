#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template};


mod controllers;


#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Template::fairing())
    .mount("/main", routes![controllers::home_controller::hello])
}