use crate::{
    auth::Token,
    entities::{prelude::*, *, self}, extarnal_api::CampusUsers,
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

pub async fn get_user_image(db: &DatabaseConnection, login: String) -> Result<Vec<u8>, DbErr> {
    let user: Option<users::Model> = Users::find_by_id(login).one(db).await?;
    let user: users::ActiveModel = user.unwrap().into();
    let vec: Vec<String> = user.r#try.unwrap().into();
    let mut path_to_image: String = String::from("./images/pool_target_").to_owned();
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
        .filter(users::Column::Student.eq(false))
        .order_by_desc(users::Column::Score)
        .all(db)
        .await
}

pub async fn speedrun_leaderboard(db: &DatabaseConnection) -> Result<Vec<users::Model>, DbErr> {
    Users::find()
        .filter(users::Column::Student.eq(false))
        .order_by_desc(users::Column::Speedrun)
        .all(db)
        .await
}

pub async fn generate_images(stud: pool_users::Model, game_mode: &str) {
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

pub async fn random_user (db: &DatabaseConnection) -> Result<entities::pool_users::Model, DbErr>{
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
    
    let student: entities::pool_users::Model = match random_user(&db).await {
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

    let add_pool_guess = game::ActiveModel {
        id: Set(2),
        login_to_find: Set(student.login.to_owned()),
        profile_pic: Set(student.profile_pic.to_owned()),
        first_name: Set(student.first_name.to_owned()),
        last_name: Set(student.last_name.to_owned()),
        ..Default::default()
    };
    
    match Game::find_by_id(2).one(db).await? {
        Some(_) => {
            add_pool_guess.clone().update(db).await?;
            println!("{:?}", add_pool_guess);
            Ok(add_pool_guess)
        },
        None => {
            Game::insert(add_pool_guess.clone()).exec(db).await?;
            println!("{:?}", add_pool_guess);
            Ok(add_pool_guess)
        }
    }
}

pub async fn get_campus_users(db: &DatabaseConnection) -> Result<Vec<pool_users::Model>, DbErr> {
    PoolUsers::find().all(db).await
}

pub async fn update_campus_user(db: &DatabaseConnection, campus_users: Vec<CampusUsers>) {
    let _ = PoolUsers::delete_many().exec(db).await;
    let mut new_user: i32 = 0;
    for i in 0..campus_users.len() {
        let record = pool_users::ActiveModel {
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
        match PoolUsers::insert(record).exec(db).await {
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