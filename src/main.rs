#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[get("/login")]
fn login() -> &'static str {
  "Login page"
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![index, login])
}