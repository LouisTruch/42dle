use rocket::http::CookieJar;
use rocket::{State, http::Status, serde::json::Json, tokio::time::sleep, form::Form};
use std::env;
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

        if alumni == true || active == false || users[i].last_name == "Angoule" || users[i].last_name == "Angouleme"{
            users.remove(i);
            continue;
        }
        i = i + 1;
    }
    users
}

#[post("/", data = "<data>")]
pub async fn game_try(data: Form<NewTry>, db: &State<DatabaseConnection>, token: Option<Token>) {
    match token {
        Some(cookie) => {
            let _ = db::update_try_by_login(
                &db, cookie.user_id, 
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

// #[get("/admin")]
// pub async fn is_admin(token: Option<Token>) -> Result<bool, Status> {
//     match token {
//         Some(login) => {
//             let admin_list: String =  env::var("ADMIN_LIST").expect("ADMIN_LIST not found in .env");
//             let admin_name: Vec<&str> = admin_list.split(";").collect();
//             if admin_name.contains(&&login.user_id.as_str()){
//                 Ok(true)
//             } else {
//                 Err(Status { code: 403 })
//             }
//         }
//         None => {
//             println!("is_admin: You are not log in.");
//             Err(Status { code: 401 })
//         }
//     }
// }
