use std::env;
use rocket::request::{FromRequest, Request};
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::serde::json::Json;
use rocket::time::{Duration, OffsetDateTime};
use serde::{Deserialize, Serialize};
use sea_orm::DatabaseConnection;
use rocket::State;
// use crate::game::{get_users_campus, CampusStudent};
use crate::student_db;
use crate::entities::users;
use crate::extarnal_api::{get_user_data, generate_token};
use rocket::request::*;

#[derive(Deserialize)]
pub struct ApiToken {
    access_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct Token{
    pub user_data: String
}

#[derive(strum_macros::Display)]
pub enum Situation {
    Stud,
    Pool
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookie_content = request
        .cookies()
        .get_private("user_data")
        .and_then(|cookie| cookie.value().parse().ok())
        .map(|data| Token{user_data: data}.user_data);
        match cookie_content {
            Some(user_data) => {
                println!("cookie: {user_data}");
                return Outcome::Success(Token { user_data: (user_data) });
            }
            None => {
                return Outcome::Failure(
                    (Status::from_code(401).unwrap(), "parsing error".to_string())
                );
            }
        }
    }
}

fn generate_cookie(value: &String, cookie: &CookieJar<'_>, name: String, situation: String) -> (){
    // Create new cookie with user_data as name and login as value
    let final_value = if name == "user_data" {
        let mut vec: Vec<String> = Vec::with_capacity(2); 
        vec.push(value.clone());
        vec.push(situation);
        vec.join(";")
    } else { value.clone() };
    let mut new_cookie = Cookie::new(name, final_value);
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

fn generate_admin_cookie(token: &String, cookie: &CookieJar<'_>, login: &String){
    // Check if the new user is an admin
    let admin_list: String =  env::var("ADMIN_LIST").expect("ADMIN_LIST not found in .env");
    let admin_name: Vec<&str> = admin_list.split(";").collect();
    if admin_name.contains(&&login.as_str()){
        generate_cookie(&token, cookie, String::from("token"), String::new());
    }
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
            println!("already a cookie for {}", cookie.user_data.split(";").next().unwrap());
            return ();
        }
        None => {}
    }

    let token = generate_token(code).await;
    let (login, img, situation) = get_user_data(token.clone()).await;
    generate_admin_cookie(&token, cookie, &login);
    generate_cookie(&login, cookie, String::from("user_data"), situation);
    match student_db::new_user(&db, &login, &img).await {
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
            let coke = jar.get_private("user_data").unwrap().clone();
            println!("Remove session cookie of {}", coke.value().to_string());
            jar.remove_private(Cookie::named("user_data"));
        }
        None => {
            println!("You can't logout");
        }
    }
}

#[get("/info")]
pub async fn get_info(token: Option<Token>, db: &State<DatabaseConnection>
) -> Result<Json<users::Model>, Status> {
    match token {
        Some(cookie) => {
            match student_db::get_user(db, cookie.user_data.split(";").next().unwrap().to_string()).await {
                Ok(res) => Ok(Json(res)),
                Err(_) => Err(Status {code: 404})
            }
        }
        None => {
            println!("You are not logged in");
            Err(Status {code: 401})
        }
    }
}

#[get("/admin")]
pub async fn is_admin(token: Option<Token>) -> Result<Json<bool>, Status> {
    match token {
        Some(login) => {
            let admin_list: String =  env::var("ADMIN_LIST").expect("ADMIN_LIST not found in .env");
            let admin_name: Vec<&str> = admin_list.split(";").collect();
            if admin_name.contains(&&login.user_data.split(";").next().unwrap()){
                Ok(Json(true))
            } else {
                Err(Status { code: 403 })
            }
        }
        None => {
            println!("is_admin: You are not log in.");
            Err(Status { code: 401 })
        }
    }
}