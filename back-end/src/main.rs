#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/ohoh")]
fn a() -> &'static str {
    "baby calme down!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, a])
}