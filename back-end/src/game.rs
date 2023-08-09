use rocket::State;
use rocket::http::CookieJar;
use sea_orm::DatabaseConnection;
use serde::{Deserialize};
use rocket::form::Form;
use crate::db;
use crate::auth::{Token, ImageData};


#[derive(FromForm)]
pub struct NewTry {
    login_to_guess: String
}

#[derive(Deserialize)]
pub struct CampusStudent {
    login: String,
    first_name: String,
    last_name: String,
    image: ImageData,
    #[serde(rename = "alumni?")]  // Rename the field to match the JSON key
    alumni: bool,
    #[serde(rename = "active?")]  // Rename the field to match the JSON key
    active: bool,
}

pub async fn get_users_campus (token: String) -> Vec<CampusStudent>{
    let mut users: Vec<CampusStudent> = Vec::new();
    let client: reqwest::Client = reqwest::Client::new();
    let mut bearer: String = String::from("Bearer ").to_owned();
    bearer.push_str(&token);

    let reponse_campus_users = client.get("https://api.intra.42.fr/v2/campus/31/users?per_page=100")
    .header("Authorization", bearer.as_str())
    .send()
    .await
    .expect("get_users_campus: Response from 42's api failed");

    let nb_users = reponse_campus_users.headers()
    .get("X-Total")
    .expect("get_users_campus token: get X-Total error");

    let nb_pages: i32 = (nb_users
    .to_str()
    .expect("get_users_campus: Can't convert the number of user from request into an int")
    .parse::<f32>()
    .unwrap() / 100.0).ceil() as i32;

    for i in 0..nb_pages{
        let mut url: String = String::from("https://api.intra.42.fr/v2/campus/31/users?per_page=100&page=X+").to_owned();
        url.push_str(&i.to_string());

        let campus_users = client.get(url)
            .header("Authorization", bearer.as_str())
            .send()
            .await
            .expect("get_users_campus: Response from 42's api failed")
            .json::<Vec<CampusStudent>>()
            .await
            .expect("get_users_campus: Parse the response from 42's api failed");

        users.extend(campus_users);
    }
    users
}


#[post("/", data = "<data>")]
pub async fn game_try(data: Form<NewTry>, db: &State<DatabaseConnection>, jar: &CookieJar<'_>, token: Option<Token>) {
    match token {
        Some(_) => {
            let coke = jar.get_private("user_id").unwrap().clone();
            db::update_try_by_login(&db, coke.value().to_string(), data.login_to_guess.clone()).await;
        }
        None => {
            println!("You are not logged in");
        }
    }
}