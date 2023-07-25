mod auth;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![auth::generate_token, auth::get_all_users])
}