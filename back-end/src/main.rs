use mysql::prelude::*;
use mysql::Pool;
use std::env;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/token/<code>")]
async fn token(code: &str) -> String {
    let client = reqwest::Client::new();
    let res = client.post("https://api.intra.42.fr/oauth/token")
        .header("grant_type", "authorization_code")
        .header("client_id", env::var("CLIENT_ID").unwrap())
        .header("client_secret", env::var("CLIENT_SECRET").unwrap())
        .header("code", code)
        .send()
        .await;

    match res {
        Ok(_res) =>{
            // print!("Ok from match: {}", _res.text().await.expect("failed"));
            format!("Ok from match: {}", _res.text().await.expect("failed"))
        }
        Err(err) =>{
            // print!("Err from match: {}", err);
            format!("Err from match: {}", err)
        }
    }
    // .header("redirect_uri", uuid)
    // "Hello, your code is ".to_string() + code
}

#[launch]
fn rocket() -> _ {


    // let link: String = format!("mysql://{}:{}@mysql/{}", env::var("MYSQL_USER").unwrap(), env::var("MYSQL_PASSWORD").unwrap(), env::var("DB_NAME").unwrap());
    // let pool = Pool::new(link.as_str()).unwrap();
    // let mut conn = pool.get_conn().unwrap();

    // conn.query_drop(
    //     r"
    //     CREATE TABLE IF NOT EXISTS users (
    //         id INT AUTO_INCREMENT PRIMARY KEY,
    //         login TEXT NOT NULL
    //     )",
    // ).unwrap();
    rocket::build().mount("/", routes![index, token])
}