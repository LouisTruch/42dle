use crate::{
    auth::{Token, Situation},
    entities::{prelude::*, *, self},
    extarnal_api::CampusUsers,
};
use image;
use rand::{rngs::StdRng, Rng, SeedableRng};
use reqwest;
use rocket::{http::Status, serde::json::Json, State};
use sea_orm::*;
use std::{
    fs::{self, File},
    io::{copy, Cursor},
};

// take bool to know if he his stud or not
pub async fn new_user(
    db: &DatabaseConnection,
    login: &String,
    profile_pic: &String,
    situation: String,
) -> Result<InsertResult<users::ActiveModel>, DbErr> {
    // Check if users is already in db
    let existing_user = Users::find_by_id(login).one(db).await?;
    if existing_user.is_some() {
        return Err(DbErr::RecordNotInserted);
    }
    let bool_situation: bool = if situation == Situation::Pool.to_string(){
         false 
    } else { true };

    // Create a record to add in users table
    let record = users::ActiveModel {
        login: Set(login.to_owned()),
        profile_pic: Set(profile_pic.to_owned()),
        student: Set(bool_situation),
        r#try: Set(vec![]),
        pokedex: Set(vec![]),
        ..Default::default()
    };
    // Insert in users tables and return the Result
    Users::insert(record).exec(db).await
}

pub async fn update_try_by_login(
    db: &DatabaseConnection,
    login: String,
    new_try: String,
    id: i8
) -> Result<users::Model, DbErr> {
    // Find users in db with login ( primary key ) and update with new try
    let users: Option<users::Model> = Users::find_by_id(login).one(db).await?;
    let mut users: users::ActiveModel = users.unwrap().into();
    let mut new_vec: Vec<String> = users.r#try.unwrap().into();
    new_vec.push(new_try.to_string());

    let game: Option<game::Model> = Game::find_by_id(id).one(db).await?; // change it after
    let mut game: game::ActiveModel = game.expect("update_try_by_login: no user to guess").into();

    // Check if try is equal to login of the day
    let find_login: String = game.login_to_find.unwrap().into();
    if new_try == find_login {
        // add score for the win !
        let nb_score: i32 = users.score.unwrap().into();
        let mut score_to_add: i32 = 12 - (new_vec.len() as i32) * 2;
        if score_to_add <= 0 {
            score_to_add = 1;
        }
        
        let mut pokedex_vec: Vec<String> = users.pokedex.unwrap().into();
        pokedex_vec.push(new_try.to_string());
        users.pokedex = Set(pokedex_vec);

        users.score = Set(nb_score + score_to_add as i32);
        users.win = Set(true);
    }
    users.r#try = Set(new_vec);
    game.login_to_find = Set(find_login);

    game.update(db).await?;
    users.update(db).await
}

pub async fn get_user_image(db: &DatabaseConnection, login: String) -> Result<Vec<u8>, DbErr> {
    let user: Option<users::Model> = Users::find_by_id(login).one(db).await?;
    let user: users::ActiveModel = user.unwrap().into();
    let vec: Vec<String> = user.r#try.unwrap().into();
    let mut path_to_image: String = String::from("./images/student_target_").to_owned();
    if vec.len() > 6 || user.win.unwrap().into() {
        path_to_image.push_str("0");
    } else {
        path_to_image.push_str(&(6 - vec.len()).to_string());
    }
    path_to_image.push_str(".jpeg");
    let image_data = fs::read(path_to_image).unwrap();
    Ok(image_data)
}

pub async fn leaderboard(db: &DatabaseConnection) -> Result<Vec<users::Model>, DbErr> {
    Users::find()
        .filter(users::Column::Student.eq(true))
        .order_by_desc(users::Column::Score)
        .all(db)
        .await
}

pub async fn generate_images(stud: student_users::Model, game_mode: &str) {
    let img_bytes = reqwest::get(stud.profile_pic)
        .await
        .expect("generate_images: Get request to 42's api for profil pic issue");
    let path_initiale = game_mode.to_string() + "/pool_target_";
    let mut first_image = path_initiale.clone();
    first_image.push_str("0.jpeg");

    fs::create_dir_all(game_mode).expect("generate_images: fail to create directory");
    let create_first_image = first_image.clone();
    let mut file =
        File::create(create_first_image).expect("generate_images: fail to create file");

    let mut content = Cursor::new(
        img_bytes
            .bytes()
            .await
            .expect("generate_images: Convert image to bytes error."),
    );
    copy(&mut content, &mut file).expect("generate_images: fail to copy data into image");

    for i in 0..7 {
        let mut path: String = path_initiale.clone();
        path.push_str(i.to_string().as_str());
        path.push_str(".jpeg");

        let mut image =
            image::open(first_image.clone()).expect("generate_images: fail to open iamge");
        image = if i > 4 { image.grayscale() } else { image };
        image = if i == 4 { image.huerotate(180) } else { image };
        image = if i == 3 { image.rotate180() } else { image };
        image = if i > 1 {
            image.blur((i * 2) as f32)
        } else {
            image
        };

        image
            .save(path)
            .expect("generate_images: fail to save image");
    }
}

pub async fn random_user (db: &DatabaseConnection) -> Result<entities::student_users::Model, DbErr>{
    let students = get_campus_users(&db).await?;
    if students == [] {
        return Err(DbErr::RecordNotFound(
            "new_day: No data in campus_users table!".to_string(),
        ));
    }
    let mut rng = StdRng::from_rng(rand::thread_rng()).unwrap();
    let index = rng.gen_range(0..students.len());
    let student = &students[index];
    Ok(student.clone())
}


pub async fn new_day(db: &DatabaseConnection) -> Result<game::ActiveModel, DbErr> {
    let student: entities::student_users::Model = match random_user(&db).await {
        Ok(stud) => { stud }
        Err(e) => {
            return Err(e);
        }
    };
    generate_images(student.clone(), "./images").await;
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

    let add_student_guess = game::ActiveModel {
        id: Set(1),
        login_to_find: Set(student.login.to_owned()),
        profile_pic: Set(student.profile_pic.to_owned()),
        first_name: Set(student.first_name.to_owned()),
        last_name: Set(student.last_name.to_owned()),
        ..Default::default()
    };
    
    match Game::find_by_id(1).one(db).await? {
        Some(_) => {
            add_student_guess.clone().update(db).await?;
            println!("{:?}", add_student_guess);
            Ok(add_student_guess)
        },
        None => {
            Game::insert(add_student_guess.clone()).exec(db).await?;
            println!("{:?}", add_student_guess);
            Ok(add_student_guess)
        }
    }
}

pub async fn get_campus_users(db: &DatabaseConnection) -> Result<Vec<student_users::Model>, DbErr> {
    StudentUsers::find().all(db).await
}

pub async fn update_campus_user(db: &DatabaseConnection, campus_users: Vec<CampusUsers>) {
    let _ = StudentUsers::delete_many().exec(db).await;
    let mut new_user: i32 = 0;
    for i in 0..campus_users.len() {
        let record = student_users::ActiveModel {
            login: Set(campus_users[i].login.to_owned()),
            profile_pic: Set(campus_users[i]
                .image
                .as_ref()
                .unwrap()
                .versions
                .as_ref()
                .unwrap()
                .medium
                .as_ref()
                .unwrap()
                .to_owned()
                .to_string()),
            first_name: Set(campus_users[i].first_name.to_owned()),
            last_name: Set(campus_users[i].last_name.to_owned()),
            ..Default::default()
        };
        match StudentUsers::insert(record).exec(db).await {
            Ok(_) => {
                new_user = new_user + 1;
            }
            Err(_) => {}
        };
    }
    if new_user > 0 {
        println!("{} users created !", new_user);
    }
}

pub async fn get_user(db: &DatabaseConnection, login: String) -> Result<users::Model, DbErr> {
    let user = Users::find_by_id(login).one(db).await?;
    let user: users::Model = user.unwrap().into();
    Ok(user)
}