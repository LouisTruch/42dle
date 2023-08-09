mod auth;
mod index;
mod db;
mod entities;
mod game;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use dotenv::dotenv;
use std::thread;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    // Connect to the database and panic if fail
    let db_conn = match Database::connect("postgresql://onverrabien:chibrax22@localhost/42DLE").await {
        Ok(db_conn) => db_conn,
        Err(e) => panic!("Error database connection: {}", e)
    };

    // Migrate the users table
    match Migrator::up(&db_conn, None).await {
        Ok(()) => println!("Migration done"),
        Err(e) => println!("Migration failed: {}", e)
    };

    // let child = thread::spawn(move || generate_target());
    // child.join();

    rocket::build()
        .manage(db_conn)
        .attach(Cors)
        .mount("/", routes![
            index::no_auth_index,
            index::index])
        .mount("/auth", routes![
            auth::init_session, 
            auth::game_try,
            auth::logout,
            auth::update_db,])
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://localhost:5173",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}