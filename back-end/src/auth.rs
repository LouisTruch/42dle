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
    // Get informations in .env file to generate request's body for 42's api
    let client_id: String =  env::var("CLIENT_ID").expect("CLIENT_ID not found in .env");
    let client_secret: String =  env::var("CLIENT_SECRET").expect("CLIENT_SECRET not found in .env");
    let data = [("grant_type", "authorization_code"),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
        ("code", &code), 
        ("redirect_uri", "http://localhost:5173/auth"),
    ];

    // Send a request to 42's API with a request body.
    // The API will return a response in JSON format.
    // This JSON response will be parsed to extract access_token and return it as a String.
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
    // Prepare the "Authorization" header by appending the token to "Bearer ".
    let mut bearer: String = String::from("Bearer ").to_owned();
    bearer.push_str(&token);

    // Send a GET request to 42's api with the "Authorization" header.
    // wait the response and parse it into the 'ApiData' struct.
    let client = reqwest::Client::new();
    let res = client.get("https://api.intra.42.fr/v2/me")
        .header("Authorization", bearer.as_str())
        .send()
        .await
        .expect("get_user_data: Response from 42's api failed")
        .json::<ApiData>()
        .await
        .expect("get_user_data: Parse the response from 42's api failed");

    // From this struct it will Extract login & image url.
    // Return them as a tuple.
    return (res.login, res.image.versions.medium);
}

/*
    1) Exchange the code provided by the  frontend's query for a token returned by 42's api
    2) This token is used to retrieve the login & pp of the user who sent the code
    3) The login & pp are sent and saved into the Postgres database
    4) Then, the login is used as value for the session cookie
    5) Finally, return the new private session cookie to the frontend
*/
#[get("/token/<code>")]
pub async fn init_session(db: &State<DatabaseConnection> ,code: &str, jar: &CookieJar<'_>) -> () {
    let token = generate_token(code).await;
    let (login, img) = get_user_data(token).await;
    // users::new_user(&db, &login, img).await.expect("Fail to create new user in db");
    let mut cookie = Cookie::new("user_id", login);
    cookie.secure();
    cookie.set_secure(false);
    cookie.set_same_site(SameSite::Lax);
    jar.add_private(cookie)
}


#[get("/logout")]
pub fn logout(_user: User, jar: &CookieJar<'_>) {
    let coke = jar.get_private("user_id").unwrap().clone();
    println!("Cookie value: {}", coke.value().to_string());
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
