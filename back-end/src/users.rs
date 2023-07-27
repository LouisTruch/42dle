use sea_orm::{DatabaseConnection, Set};
use sea_orm::*;
use crate::entities::{prelude::*, *};

pub async fn new_user(
    db: &DatabaseConnection,
    login: &String, profile_pic: String
) -> Result<users::ActiveModel, DbErr> {
    println!("login: {login}, pp: {profile_pic}");
    users::ActiveModel {
        login: Set(login.to_owned()),
        profile_pic: Set(profile_pic.to_owned()),
        score: Set(Some(0)),
        ..Default::default()
    }
    .save(db)
    .await
}

pub async fn update_user_by_login(
    db: &DatabaseConnection,
    data: users::Model,
) -> Result<users::Model, DbErr> {
    let users: users::ActiveModel = Users::find_by_id(data.login)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
        .map(Into::into)?;

    users::ActiveModel {
        login: users.login,
        score: Set(data.score.to_owned()),
        profile_pic: Set(data.profile_pic.to_owned()),
    }
    .update(db)
    .await
}