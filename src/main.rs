#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;
use serde::{Serialize};

#[derive(Serialize)]
struct Context {
  title: &'static str
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &Context { title: "Hello, World!" })
}

#[get("/login")]
fn login() -> Template {
  Template::render("index", &Context { title: "Login page" })
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![index, login]).attach(Template::fairing())
}