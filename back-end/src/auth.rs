use std::env;
use rocket::request::{FromRequest, Request};
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::time::{Duration, OffsetDateTime};
use serde::{Deserialize, Serialize};
use sea_orm::DatabaseConnection;
use rocket::State;
use crate::{db, game};
use rocket::request::*;

#[derive(Deserialize)]
pub struct ApiToken {
    access_token: String,
}

#[derive(Deserialize)]
pub struct ImageData {
    // link: String,
    versions: ImageVersions,
}

#[derive(Deserialize)]
pub struct ImageVersions {
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

#[derive(Deserialize, Serialize)]
pub struct Token{user_id: String}

/*
    CODE BELOW
*/

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookie_content = request
        .cookies()
        .get_private("user_id")
        .and_then(|cookie| cookie.value().parse().ok())
        .map(|id| Token{user_id: id}.user_id);
        match cookie_content {
            Some(user_id) => {
                return Outcome::Success(Token { user_id: (user_id) });
            }
            None => {
                return Outcome::Failure(
                    (Status::from_code(401).unwrap(), "parsing error".to_string())
                );
            }
        }
    }
}

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

fn generate_cookie(login: &String, cookie: &CookieJar<'_>) -> (){
    // Create new cookie with user_id as name and login as value
    let mut new_cookie = Cookie::new("user_id", login.clone());
    // set cookie to be lax with SameSite
    // new_cookie.secure();
    new_cookie.set_secure(false);
    new_cookie.set_same_site(SameSite::Lax);
    // set an expirassion of 1 hour to the cookie
    assert_eq!(new_cookie.expires(), None);
    let mut now = OffsetDateTime::now_utc();
    now += Duration::hours(1);
    new_cookie.set_expires(now);
    assert!(new_cookie.expires().is_some());
    // add cookie to cookie jar <3
    cookie.add_private(new_cookie);
}

/*
    1) Exchange the code provided by the  frontend's query for a token returned by 42's api
    2) This token is used to retrieve the login & pp of the user who sent the code
    3) The login & pp are sent and saved into the Postgres database
    4) Then, the login is used as value for the session cookie
    5) Finally, return the new private session cookie to the frontend
*/
#[get("/token/<code>")]
pub async fn init_session(token: Option<Token>, db: &State<DatabaseConnection>, code: &str, cookie: &CookieJar<'_>) -> () {
    match token {
        Some(cookie) => {
            //Need to be removed
            // let a = game::generate_target();
            // a.await;
            println!("ALREADY A COOKIE FOR: {}", cookie.user_id);
            return ();
        }
        None => {
            println!("CREATE NEW COOKIE");
        }
    }

    let token = generate_token(code).await;
    println!("TOKen: {token}");
    let (login, img) = get_user_data(token.clone()).await;
    generate_cookie(&login, cookie);
    match db::new_user(&db, &login, &img).await {
        Ok(_) => println!("{login} was created in db"),
        Err(_e) => {
            println!("init_session: {_e}");
            return ();
        }
    };
}

#[get("/logout")]
pub fn logout(jar: &CookieJar<'_>, token: Option<Token>) {
    match token {
        Some(_) => {
            let coke = jar.get_private("user_id").unwrap().clone();
            println!("Remove session cookie of {}", coke.value().to_string());
            jar.remove_private(Cookie::named("user_id"));
        }
        None => {
            println!("You can't logout");
        }
    }
}
