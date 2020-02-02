#![feature(proc_macro_hygiene, decl_macro)]
use rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn start() {
    rocket::ignite().mount("/", routes![index]).launch();
}