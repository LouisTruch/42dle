#[macro_use]
extern crate rocket;

mod auth;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            auth::exchange_code, 
            auth::get_all_users,
            auth::no_auth_index,
            auth::index,
            auth::post_login,
            auth::quit,])
}