#[macro_use]
extern crate rocket;

mod auth;
mod index;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            index::no_auth_index,
            index::index])
        .mount("/auth", routes![
            auth::exchange_code, 
            auth::get_all_users,
            auth::post_login,
            auth::quit,])
}