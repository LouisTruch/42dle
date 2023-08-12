mod auth;
mod index;
mod student_db;
mod entities;
mod game;
mod extarnal_api;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use dotenv::dotenv;
// use std::thread::{sleep, self};
// use chrono::{Local, Duration};
// use std::time::Duration;
use rocket::tokio::time::{sleep, Duration};


#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Warn)
        .init();
    dotenv().ok();
    // Connect to the database and panic if fail
    let db_conn = match Database::connect("postgresql://onverrabien:chibrax22@localhost/42DLE").await {
        Ok(db_conn) => db_conn,
        Err(e) => panic!("Error database connection: {}", e)
    };

    // Migrate the users table
    // match Migrator::down(&db_conn, None).await {
    //     Ok(()) => println!("Migration down done"),
    //     Err(e) => println!("Migration down failed: {}", e)
    // };

    match Migrator::up(&db_conn, None).await {
        Ok(()) => println!("Migration done"),
        Err(e) => println!("Migration failed: {}", e)
    };

    let db_clone: DatabaseConnection = db_conn.clone();
    tokio::spawn(daily_interval(db_clone));
  
    rocket::build()
        .manage(db_conn)
        .attach(Cors)
        .mount("/", routes![
            index::no_auth_index,
            index::index])
        .mount("/auth", routes![
            auth::init_session, 
            auth::logout,
            auth::get_info,
            auth::is_admin,
            ])
        .mount("/game", routes![
            game::game_try,
            game::update_pool_db,
            game::update_student_db,
            game::new_target,
            game::get_guess_image,
            game::get_leaderboard,
            student_db::get_all_users,
        ])

}

async fn daily_interval(db: DatabaseConnection) {
    loop {
        // let time_now = Local::now();

        // let next_midnight = (time_now + Duration::days(1)).date().and_hms(0, 0, 0);

        // let duration = next_midnight.signed_duration_since(time_now).to_std().unwrap();

        // sleep(duration);
        sleep(Duration::from_millis(20000)).await;
        println!("NEW TARGET GENERATED");
        {
            match student_db::new_day(&db).await {
                Ok(_) => {},
                Err(e) => {println!("daily_interval: {e}");}
            } 
        }
    }
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