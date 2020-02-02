use rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn start() {
    rocket::ignite().mount("/", routes![index]).launch();
}