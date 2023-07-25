use mysql::prelude::*;
use mysql::Pool;
use std::env;

#[macro_use] extern crate rocket;

#[get("/token/<code>")]
async fn generate_token(code: &str) -> String {
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
            format!("generateToken: {}", _res.text().await.expect("failed"))
        }
        Err(err) =>{
            format!("Error in generateToken: {}", err)
        }
    }
}

#[get("/users")]
async fn get_all_users() -> String {
    let mut bearer: String = String::from("Bearer ").to_owned();
    let tmp: String = String::from("Bon").to_owned();
    bearer.push_str(&tmp);
    let client = reqwest::Client::new();

    let res = client.get("https://api.intra.42.fr/v2/users")
        .header("Authorization", bearer.as_str())
        .send()
        .await;

    match res {
        Ok(_res) =>{
            format!("getAllUsers: {}", _res.text().await.expect("failed"))
        }
        Err(err) =>{
            format!("Error in getAllUsers: {}", err)
        }
    }
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
    rocket::build().mount("/", routes![generate_token, get_all_users])
}