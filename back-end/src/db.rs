use rocket::{State, http::{CookieJar, Status}, serde::json::Json};
use sea_orm::*;
use rand::{Rng, rngs::StdRng, SeedableRng};
use reqwest;
use image;
use crate::{entities::{prelude::*, *}, game::CampusStudent, auth::Token};

pub async fn new_user(
    db: &DatabaseConnection,
    login: &String, 
    profile_pic: &String,
    ) -> Result<InsertResult<users::ActiveModel>, DbErr> {  

    // Check if users is already in db
    let existing_user = Users::find_by_id(login).one(db).await?;
    if existing_user.is_some() {
        return Err(DbErr::RecordNotInserted);
    }
    
    // Create a record to add in users table
    let record = users::ActiveModel {
        login: Set(login.to_owned()),
        profile_pic: Set(profile_pic.to_owned()),
        r#try: Set(vec![]),
        ..Default::default()
    };

    // Insert in users tables and return the Result
    Users::insert(record).exec(db).await
}


pub async fn update_try_by_login(
    db: &DatabaseConnection,
    login: String,
    new_try: String
    ) -> Result<users::Model, DbErr> {

    // Find users in db with login ( primary key ) and update with new try
    let users: Option<users::Model> = Users::find_by_id(login).one(db).await?;
    let mut users: users::ActiveModel = users.unwrap().into();
    let mut new_vec: Vec<String> = users.r#try.unwrap().into();
    new_vec.push(new_try.to_string());

    let game: Option<game::Model> = Game::find_by_id(1).one(db).await?; // change it after
    let mut game: game::ActiveModel = game.unwrap().into();
    
    // Check if try is equal to login of the day
    let find_login: String = game.login_to_find.unwrap().into();
    if new_try == find_login {

        // add score for the win !
        let nb_score: i32 = users.score.unwrap().into();
        let mut score_to_add: usize = 11 - new_vec.len();
        if score_to_add == 0 {
            score_to_add = 1;
        }
        users.score = Set(nb_score + score_to_add as i32);
        users.win = Set(true);
    }
    users.r#try = Set(new_vec);
    game.login_to_find = Set(find_login);

    game.update(db).await?;
    users.update(db).await
}


#[get("/users")]
pub async fn get_all_users(
    token: Option<Token>,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<users::Model>>, Status> {
    match token {
        Some(_) => {
            let db: &DatabaseConnection = &db;
            match Users::find().all(db).await {
                Ok(result) => Ok(Json(result)),
                Err(_) => Err(Status { code: 404 })
            }
        }
    None => {
            println!("You are not logged in");
            Err(Status {code: 401})
        }
    }
}

pub async fn get_leaderboard(
    db: &DatabaseConnection,
) -> Result<Vec<users::Model>, DbErr> {

    Users::find()
        .order_by_desc(users::Column::Score)
        .all(db)
        .await
}

fn generate_images(stud: campus_users::Model){
    let img_bytes = reqwest::blocking::get(stud.profile_pic)
    .expect("generate_images: Get request to 42's api for profil pic issue")
    .bytes()
    .expect("generate_images: Failure to convert image's reponse to bytes.");
 
    let image = image::load_from_memory(&img_bytes)
    .expect("generate_images: Fail to load image from memory");
}

pub async fn new_day(
    db: &DatabaseConnection,
) -> Result<InsertResult<game::ActiveModel>, DbErr> {

    let students = get_campus_users(&db).await.expect("new_target: Error in parsing of get_campus_users's return");
    let mut rng = StdRng::from_rng(rand::thread_rng()).unwrap();
    let index = rng.gen_range(0..students.len());

    generate_images(students[index].clone());
    // Get all users
    let users: Vec<users::Model> = Users::find().all(db).await?;

    // Reset win and try for all users
    for user in users {
        let user: Option<users::Model> = Users::find_by_id(user.login).one(db).await?;
        let mut user: users::ActiveModel = user.unwrap().into();
        user.win = Set(false);
        user.r#try = Set(vec![]);
        user.update(db).await?;
    }

    // Create a new user to guess to add in game tables
    println!("{:?}", students[index]);
    let new_day = game::ActiveModel {
        login_to_find: Set(students[index].login.to_owned()),
        profile_pic: Set(students[index].profile_pic.to_owned()),
        first_name: Set(students[index].first_name.to_owned()),
        last_name: Set(students[index].last_name.to_owned()),
        ..Default::default()
    };

    // Insert in game tables and return the Result
    Game::insert(new_day).exec(db).await
}

pub async fn get_campus_users(
    db: &DatabaseConnection
) ->  Result<Vec<campus_users::Model>, DbErr> {
    
    CampusUsers::find()
        .all(db)
        .await
}

pub async fn update_campus_user(
    db: &DatabaseConnection,
    campus_users: Vec<CampusStudent>
) {
    let mut new_user: i32 = 0;
    for i in 0..campus_users.len() {
        let record = campus_users::ActiveModel {
            login: Set(campus_users[i].login.to_owned()),
            profile_pic: Set(
                campus_users[i]
                .image.as_ref().unwrap()
                .versions.as_ref().unwrap()
                .medium.as_ref().unwrap().to_owned().to_string()
            ),
            first_name: Set(campus_users[i].first_name.to_owned()),
            last_name: Set(campus_users[i].last_name.to_owned()),
            ..Default::default()
        }; 
        match CampusUsers::insert(record).exec(db).await {
            Ok(_) => { println!("Update Campus User --> User add!"); new_user = new_user + 1;},
            Err(_) => println!("Update Campus User --> User already in db")
        };
    }
    if new_user > 0 {
        println!("{} New Users !", new_user);
    }
}

pub async fn get_user(
    db: &DatabaseConnection,
    login: String
) -> Result<users::Model, DbErr> {
    
    let user = Users::find_by_id(login).one(db).await?;
    let user: users::Model = user.unwrap().into();
    Ok(user)
}