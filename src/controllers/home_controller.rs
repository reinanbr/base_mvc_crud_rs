
use rocket_dyn_templates::{Template, context};


#[get("/<name>")]
pub fn hello(name: &str) -> Template{
    Template::render("tera/pages/home", context! { user_name: name })
}