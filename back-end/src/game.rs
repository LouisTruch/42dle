use rocket::State;
use rocket::http::CookieJar;
use std::time::Duration;
use rocket::tokio::time::sleep;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use rocket::form::Form;
use crate::db;
use crate::auth::Token;


#[derive(FromForm)]
pub struct NewTry {
    login_to_guess: String
}

#[derive(Deserialize)]
pub struct ImageData {
    pub versions: Option<ImageVersions>,
}

#[derive(Deserialize)]
pub struct ImageVersions {
    pub medium: Option<String>,
}

#[derive(Deserialize)]
pub struct CampusStudent {
    pub login: String,
    pub first_name: String,
    pub last_name: String,
    pub image: Option<ImageData>,
    #[serde(rename = "alumni?")]  // Rename the field to match the JSON key
    alumni: Option<bool>,
    #[serde(rename = "active?")]  // Rename the field to match the JSON key
    active: Option<bool>,
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

    println!("{}", bearer);
    for i in 1..=nb_pages{
        let mut url: String = String::from("https://api.intra.42.fr/v2/campus/31/users?per_page=100&page=").to_owned();
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
        sleep(Duration::from_millis(600)).await;
    }
    let mut i: usize = 0;
    while i < users.len(){
        let alumni = match users[i].alumni {
            Some(val) => {val},
            None => {true}
        };
        let active = match users[i].active {
            Some(val) => {val},
            None => {false}
        };
        match &users[i].image {
            Some(img_data) => {
                match &img_data.versions {
                    Some(version) => {
                        match &version.medium {
                            Some(_img) => {},
                            None => { 
                                users.remove(i);
                                continue;
                            }
                        }
                    },
                    None => { 
                        users.remove(i);
                        continue;
                    }
                }
            },
            None => { 
                users.remove(i);
                continue;
            }
        };

        if alumni == true || active == false{
            users.remove(i);
            continue;
        }
        i = i + 1;
    }
    users
}

#[post("/", data = "<data>")]
pub async fn game_try(data: Form<NewTry>, db: &State<DatabaseConnection>, jar: &CookieJar<'_>, token: Option<Token>) {
    match token {
        Some(_) => {
            let coke = jar.get_private("user_id").unwrap().clone();
            let _ = db::update_try_by_login(
                &db, coke.value().to_string(), 
                data.login_to_guess.clone()
            ).await;
        }
        None => {
            println!("You are not logged in");
        }
    }
}

#[get("/update-db")]
pub async fn update_db(token: Option<Token>, db: &State<DatabaseConnection>, jar: &CookieJar<'_>) {
    match token {
        Some(_login) => {
            let api42token: String = jar.get_private("token").unwrap().clone().value().to_string();
            let users_campus: Vec<CampusStudent> = get_users_campus(api42token).await;
            db::update_campus_user(&db, users_campus).await;
        }
        None => {
            println!("You are not log in.");
        }
    }
}