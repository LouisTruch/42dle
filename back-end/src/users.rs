use sea_orm::*;
use crate::entities::{prelude::*, *};

pub async fn new_user(
    db: &DatabaseConnection,
    login: &String, profile_pic: &String
    ) -> Result<InsertResult<users::ActiveModel>, DbErr> {  

    // Create a record to add in db
    let record = users::ActiveModel {
        login: Set(login.to_owned()),
        profile_pic: Set(profile_pic.to_owned()),
        score: Set(Some(0)),
        r#try: Set([].to_vec()),
        ..Default::default()
    };

    // Insert in db and return the Result
    Users::insert(record).exec(db).await
}

pub async fn update_score_by_login(
    db: &DatabaseConnection,
    login: &String,
    new_score: &i32
    ) -> Result<users::Model, DbErr> {

    // Find users in db with login ( primary key ) and update with new score
    let users: Option<users::Model>  = Users::find_by_id(login).one(db).await?;
    let mut users: users::ActiveModel = users.unwrap().into();
    users.score = Set(Some(*new_score));
    users.update(db).await
}

pub async fn update_try_by_login(
    db: &DatabaseConnection,
    login: &String,
    new_try: &String
    ) -> Result<users::Model, DbErr> {

    // Find users in db with login ( primary key ) and update with new try
    let users: Option<users::Model>  = Users::find_by_id(login).one(db).await?;
    let mut users: users::ActiveModel = users.unwrap().into();
    // let try_vec: Vec<String> = users.r#try;
    // try_vec.push(new_try.to_string());
    // users.r#try = try_vec;
    users.update(db).await
}