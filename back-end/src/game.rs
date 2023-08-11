use reqwest::Response;
use rocket::http::CookieJar;
use rocket::{State, http::Status, serde::json::Json, tokio::time::sleep, form::Form};
use std::time::Duration;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::db;
use crate::auth::Token;
use crate::entities::users;


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

fn get_numbers_pages(campus_users: &Response) -> i32 {
    let nb_users = campus_users.headers()
    .get("X-Total")
    .expect("get_users_campus token: get X-Total error");

    (nb_users
    .to_str()
    .expect("get_users_campus: Can't convert the number of user from request into an int")
    .parse::<f32>()
    .unwrap() / 100.0).ceil() as i32

}

pub async fn get_users_campus (token: String) -> Vec<CampusStudent>{
    let mut users: Vec<CampusStudent> = Vec::new();
    let client: reqwest::Client = reqwest::Client::new();
    let mut bearer: String = String::from("Bearer ").to_owned();
    bearer.push_str(&token);

    // iter on while users remain
    let mut nb_pages= 2;
    let mut i = 1;
    while i <= nb_pages{
        let mut url: String = String::from("https://api.intra.42.fr/v2/campus/31/users?per_page=100&page=").to_owned();
        url.push_str(&i.to_string());
        let campus_users = client.get(url)
            .header("Authorization", bearer.as_str())
            .send()
            .await
            .expect("get_users_campus: Response from 42's api failed");
        // on the first request, parse X-Total to get the number of pages
        nb_pages = if i == 1 { get_numbers_pages(&campus_users) } else { nb_pages };
        // parse reponse as json struct with user's data
        let new_page = campus_users
            .json::<Vec<CampusStudent>>()
            .await
            .expect("get_users_campus: Parse the response from 42's api failed");

        users.extend(new_page);
        sleep(Duration::from_millis(600)).await;
        i += 1;
    }

    // remove inactive & alumni users
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

        if alumni == true || active == false || users[i].last_name == "Angoule" || users[i].last_name == "Angouleme"{
            users.remove(i);
            continue;
        }
        i = i + 1;
    }
    users
}

#[post("/", data = "<data>")]
pub async fn game_try(data: Form<NewTry>, db: &State<DatabaseConnection>, token: Option<Token>
) -> Result<Json<users::Model>, Status> {
    match token {
        Some(cookie) => {
            match db::update_try_by_login(&db, cookie.user_id, data.login_to_guess.clone()).await {
                Ok(res) => {
                    if res.win == true {
                        Ok(Json(res))
                    }
                    else {
                        Err(Status { code: 404 })
                    }
                },
                Err(_) => Err(Status { code: 404 })
            }
        }
        None => {
            println!("You are not logged in");
            Err(Status { code: 401 })
        }
    }
}

#[get("/update-db")]
pub async fn update_db(token: Option<Token>, db: &State<DatabaseConnection>, jar: &CookieJar<'_>) {
    match token {
        Some(_cookie) => {
            let coke = jar.get_private("token").unwrap().clone();
            let users_campus: Vec<CampusStudent> = get_users_campus(coke.value().to_string()).await;
            db::update_campus_user(&db, users_campus).await;
        }
        None => {
            println!("You are not log in.");
        }
    }
}

#[get("/new-target")]
pub async fn new_target(token: Option<Token>, db: &State<DatabaseConnection>) {
    match token {
        Some(_login) => {
            match db::new_day(&db).await {
                Ok(_) => {},
                Err(e) => {
                    println!("new_target: {e}");
                }
            }
        }
        None => {
            println!("You are not log in.");
        }
    }
}

#[get("/guess-image")]
pub async fn get_guess_image(token: Option<Token>, db: &State<DatabaseConnection>) -> Result<Vec<u8>, Status> {
    match token {
        Some(cookie) => {
            match db::get_user_image(&db, cookie.user_id).await {
                Ok(res) => Ok(res),
                Err(_) => {
                    println!("get_guess_image: failed to load image");
                    Err(Status { code: 404 })
                }
            }
        }
        None => {
            println!("get_guess_image: You are not log in.");
            Err(Status { code: 401 })
        }
    }
}

#[get("/leaderboard")]
pub async fn get_leaderboard(token: Option<Token>, db: &State<DatabaseConnection>
) -> Result<Json<Vec<users::Model>>, Status> {
    match token {
        Some(_login) => {
            match db::leaderboard(db).await {
                Ok(res) => Ok(Json(res)),
                Err(_) => Err(Status { code: 404 })
            }
        }
        None => {
            println!("get_leaderboard: You are not log in.");
            Err(Status { code: 401 })
        }
    }
}

