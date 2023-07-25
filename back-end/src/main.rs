use mysql::prelude::*;
use mysql::Pool;
use std::env;

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
    let link: String = format!("mysql://{}:{}@mysql/{}", env::var("MYSQL_USER").unwrap(), env::var("MYSQL_PASSWORD").unwrap(), env::var("DB_NAME").unwrap());
    let pool = Pool::new(link.as_str()).unwrap();
    let mut conn = pool.get_conn().unwrap();

    conn.query_drop(
        r"
        CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            login TEXT NOT NULL
        )",
    ).unwrap();

    rocket::build().mount("/", routes![index, a])
}