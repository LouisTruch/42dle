mod auth;
mod users;
mod entities;
use sea_orm::Database;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    let db_conn = match Database::connect("postgresql://onverrabien:chibrax22@localhost/42DLE").await {
        Ok(db_conn) => db_conn,
        Err(e) => panic!("Error database connection: {}", e)
    };
    rocket::build()
        .manage(db_conn)
        .mount("/", routes![
            auth::exchange_code, 
            auth::get_all_users,
            auth::no_auth_index,
            auth::index,
            auth::post_login,
            auth::quit,])
        .mount("/user", routes![
            users::new_user
        ])
}