use rocket::http::CookieJar;
use rocket::{State, http::Status, serde::json::Json, form::Form};
use sea_orm::DatabaseConnection;

use crate::auth::{Token, Situation};
use crate::{student_db, pool_db};
use crate::entities::{users, student_users, pool_users};
use crate::extarnal_api::{CampusUsers, get_students};


#[derive(FromForm)]
pub struct NewTry {
    login_to_guess: String
}


#[post("/", data = "<data>")]
pub async fn game_try(data: Form<NewTry>, db: &State<DatabaseConnection>, token: Option<Token>
) -> Result<Json<users::Model>, Status> {
    match token {
        Some(cookie) => {
            let id : i8 = if cookie.user_data.split(";").last().unwrap().to_string() == Situation::Stud.to_string() { 1 } else { 2 };
            match student_db::update_try_by_login(&db, cookie.user_data.split(";").next().unwrap().to_string(), data.login_to_guess.clone(), id).await {
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

#[get("/update-student-db")]
pub async fn update_student_db(token: Option<Token>, db: &State<DatabaseConnection>, jar: &CookieJar<'_>) {
    match token {
        Some(_cookie) => {
            let coke = jar.get_private("token").unwrap().clone();
            let users_campus: Vec<CampusUsers> = get_students(coke.value().to_string(), Situation::Stud.to_string()).await;
            student_db::update_campus_user(&db, users_campus).await;
        }
        None => {
            println!("You are not log in.");
        }
    }
}

#[get("/update-pool-db")]
pub async fn update_pool_db(token: Option<Token>, db: &State<DatabaseConnection>, jar: &CookieJar<'_>) {
    match token {
        Some(_cookie) => {
            let coke = jar.get_private("token").unwrap().clone();
            let users_campus: Vec<CampusUsers> = get_students(coke.value().to_string(), Situation::Pool.to_string()).await;
            pool_db::update_campus_user(&db, users_campus).await;
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
            if let Err(e) = student_db::new_day(&db).await {
                    println!("new_target: {e}");
            } 
            if let Err(e) = pool_db::new_day(&db).await {
                println!("new_target: {e}");
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
            if cookie.user_data.split(";").last().unwrap() == Situation::Stud.to_string(){
                match student_db::get_user_image(&db, cookie.user_data.split(";").next().unwrap().to_string()).await {
                        Ok(res) => Ok(res),
                        Err(_) => {
                            println!("get_guess_image: failed to load image");
                            Err(Status { code: 404 })
                        }
                    }
            } else {
                match pool_db::get_user_image(&db, cookie.user_data.split(";").next().unwrap().to_string()).await {
                    Ok(res) => Ok(res),
                    Err(_) => {
                        println!("get_guess_image: failed to load image");
                        Err(Status { code: 404 })
                    }
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
        Some(cookie) => {
            if cookie.user_data.split(";").last().unwrap() == Situation::Stud.to_string(){
                match student_db::leaderboard(db).await {
                    Ok(res) => Ok(Json(res)),
                    Err(_) => Err(Status { code: 404 })
                }
            } else {
                match pool_db::leaderboard(db).await {
                    Ok(res) => Ok(Json(res)),
                    Err(_) => Err(Status { code: 404 })
                }
            }
        }
        None => {
            println!("get_leaderboard: You are not log in.");
            Err(Status { code: 401 })
        }
    }
}

#[get("/student-users")]
pub async fn get_student_users(
    token: Option<Token>,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<student_users::Model>>, Status> {
    match token {
        Some(_cookie) => {
            match student_db::get_campus_users(db).await {
                Ok(result) => Ok(Json(result)),
                Err(_) => Err(Status { code: 404 }),
            }
        }
        None => {
            println!("You are not logged in");
            Err(Status { code: 401 })
        }
    }
}

#[get("/pool-users")]
pub async fn get_pool_users(
    token: Option<Token>,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<pool_users::Model>>, Status> {
    match token {
        Some(_cookie) => {
            match pool_db::get_campus_users(db).await {
                Ok(result) => Ok(Json(result)),
                Err(_) => Err(Status { code: 404 }),
            }
        }
        None => {
            println!("You are not logged in");
            Err(Status { code: 401 })
        }
    }
}

#[get("/init-speedrun")]
pub async fn init_speedrun(
    token: Option<Token>,
    db: &State<DatabaseConnection>,
)  {
    if let Some(cookie) = token {
        if cookie.user_data.split(";").last().unwrap() == Situation::Pool.to_string(){
            let time_initiale = chrono::Utc::now().timestamp();
            // student_db::begin_speedrun(db, cookie.user_data.split(";").next().unwrap(), time_initiale);
            let pool_user: pool_users::Model = pool_db::random_user(&db).await.unwrap();
            pool_db::generate_images(pool_user, "./speedrun").await;
        } else {
            let time_initiale = chrono::Utc::now().timestamp();
            // student_db::begin_speedrun(db, cookie.user_data.split(";").next().unwrap(), time_initiale);
            let stud: student_users::Model = student_db::random_user(&db).await.unwrap();
            student_db::generate_images(stud, "./speedrun").await;
        }
    } else { println!("You are not logged in"); };
}