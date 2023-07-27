use std::env;
use reqwest::Response;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::http::{Cookie, CookieJar, SameSite};
use serde::Deserialize;
use http;
use sea_orm::DatabaseConnection;
use rocket::State;
use crate::index;
use crate::users;
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

#[derive(Deserialize)]
struct ApiToken {
    access_token: String,
}

#[derive(Deserialize)]
struct ImageData {
    // link: String,
    versions: ImageVersions,
}

#[derive(Deserialize)]
struct ImageVersions {
    // large: String,
    medium: String,
    // small: String,
    // micro: String,
}

#[derive(Deserialize)]
struct ApiData {
    login: String,
    image: ImageData,
}

/*
    CODE BELOW
*/

async fn generate_token(code: &str, ) -> String{
    //get informations in .env file to generate request's body for 42's api
    let client_id: String =  env::var("CLIENT_ID").expect("CLIENT_ID not found in .env");
    let client_secret: String =  env::var("CLIENT_SECRET").expect("CLIENT_SECRET not found in .env");
    let data = [("grant_type", "authorization_code"),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
        ("code", &code), 
        ("redirect_uri", "http://localhost:5173/auth"),
    ];

    let client: reqwest::Client = reqwest::Client::new();
    let access_token = client.post("https://api.intra.42.fr/oauth/token")
        .header("Content-Type","application/x-www-form-urlencoded")
        .form(&data)
        .send()
        .await
        .expect("generate_token: Response from 42's api failed")
        .json::<ApiToken>()
        .await
        .expect("generate_token: Parse the response from 42's api failed")
        .access_token;
    access_token
}

pub async fn get_user_data(token: String) -> (String, String) {
    let mut bearer: String = String::from("Bearer ").to_owned();
    bearer.push_str(&token);

    let client = reqwest::Client::new();

    let res = client.get("https://api.intra.42.fr/v2/me")
        .header("Authorization", bearer.as_str())
        .send()
        .await
        .expect("get_user_data: Response from 42's api failed")
        .json::<ApiData>()
        .await
        .expect("get_user_data: Parse the response from 42's api failed");

    return (res.login, res.image.versions.medium);
}

#[get("/token/<code>")]
pub async fn init_session(db: &State<DatabaseConnection> ,code: &str, jar: &CookieJar<'_>) -> () {
    let token = generate_token(code).await;
    let (login, img) = get_user_data(token).await;
    // users::new_user(&db, &login, &img).await.expect("Fail to create new user in db");
    let mut cookie = Cookie::new("user_id", login);
    cookie.secure();
    cookie.set_secure(false);
    cookie.set_same_site(SameSite::Lax);
    jar.add_private(cookie)
    // // samesite
    // // "Set-Cookie".to_string()
    // http::Response::builder()
    //     .status(http::StatusCode::SEE_OTHER)
    //     .header("Location", "/")
    //     .header("Set-Cookie", added)
    //     .unwrap()
}


#[get("/logout")]
pub fn logout(_user: User, jar: &CookieJar<'_>) {
    if let Some(cookie) = jar.get("user_id") {
        let cookie_value = cookie.value();
        println!("Cookie content before deletion: {}", cookie_value);
    }
    jar.remove_private(Cookie::named("user_id"));
}


#[get("/aaa")]
pub async fn tmp(jar: &CookieJar<'_>) -> () {
    let mut cookie = Cookie::new("user_id", "armand".to_string());
    cookie.secure();
    cookie.set_secure(false);
    cookie.set_same_site(SameSite::Lax);
    jar.add_private(cookie)
}
