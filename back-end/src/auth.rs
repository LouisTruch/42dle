use std::env;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::Redirect;
use rocket::http::{Cookie, CookieJar};

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
    let res = client.post("https://api.intra.42.fr/oauth/token")
        .header("grant_type", "authorization_code")
        .header("client_id", env::var("CLIENT_ID").expect("CLIENT_ID not found in .env"))
        .header("client_secret", env::var("CLIENT_SECRET").expect("CLIENT_SECRET not found in .env"))
        .header("code", code)
        .send()
        .await;

    match res {
        Ok(_res) =>{
            format!("exchange_code: {}", _res.text().await.expect("failed"))
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

#[get("/auth/users")]
pub async fn get_all_users() -> String {
    let token_plus_tard: String = String::from("Bon").to_owned();
    let mut bearer: String = String::from("Bearer ").to_owned();
    let client = reqwest::Client::new();

    bearer.push_str(&token_plus_tard);

    let res = client.get("https://api.intra.42.fr/v2/users")
        .header("Authorization", bearer.as_str())
        .send()
        .await;

    match res {
        Ok(_res) =>{
            format!("get_all_users: {}", _res.text().await.expect("failed"))
        }
        Err(err) =>{
            format!("Error in get_all_users: {}", err)
        }
    }
}