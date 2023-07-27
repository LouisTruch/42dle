use std::env;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::Redirect;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;

use crate::index;
pub struct User(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, Self::Error> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(User)
            .or_forward(())
    }
}

#[get("/auth/token/<code>")]
pub async fn exchange_code(code: &str) -> String {
    let client: reqwest::Client = reqwest::Client::new();

    let mut client_id: String = String::from("client_id=").to_owned();
    let tmp: String =  env::var("CLIENT_ID").expect("CLIENT_ID not found in .env");
    client_id.push_str(&tmp);
    println!("{}", client_id);

    let mut client_secret: String = String::from("client_secret=").to_owned();
    let tmp2: String =  env::var("CLIENT_SECRET").expect("CLIENT_SECRET not found in .env");
    client_secret.push_str(&tmp2);
    println!("{}", client_secret);

    let mut code_to_body: String = String::from("code=").to_owned();
    code_to_body.push_str(&code);
    println!("{}", code_to_body);

    let data = [("grant_type", "authorization_code"),
    ("client_id", &tmp),
    ("client_secret", &tmp2),
    ("code", &code), 
    ("redirect_uri", "http://localhost:5173/auth")];

    let res = client.post("https://api.intra.42.fr/oauth/token")
        .header("Content-Type","application/x-www-form-urlencoded")
        .form(&data)
        .send()
        .await;

    match res {
        Ok(_res) =>{
            _res.text().await.expect("failed")
        }
        Err(err) =>{
            format!("Error in exchange_code: {}", err)
        }
    }
}

#[get("/auth/login")]
pub fn post_login(jar: &CookieJar<'_>) -> Redirect {
    println!("generate new cookie");
    jar.add_private(Cookie::new("user_id", 1.to_string()));
    Redirect::to(uri!(index::index))
}

#[get("/auth/quit")]
pub fn quit(_user: User, jar: &CookieJar<'_>) -> Redirect  {
    jar.remove_private(Cookie::named("user_id"));
    Redirect::to(uri!(index::index))
}

#[get("/auth/users/<code>")]
pub async fn get_all_users(code: &str) -> String {
    let mut bearer: String = String::from("Bearer ").to_owned();

    bearer.push_str(&code);
    let client = reqwest::Client::new();

    let res = client.get("https://api.intra.42.fr/v2/me")
        .header("Authorization", bearer.as_str())
        .send()
        .await;

    match res {
        Ok(_res) =>{
            _res.text().await.expect("failed")
        }
        Err(err) =>{
            format!("Error in get_all_users: {}", err)
        }
    }
}