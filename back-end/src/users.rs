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
        ..Default::default()
    };

    // Insert in db and return the Result
    Users::insert(record).exec(db).await
}

pub async fn update_user_by_login(
    db: &DatabaseConnection,
    data: users::Model,
    ) -> Result<users::Model, DbErr> {

    // Find users in db with login ( primary key ) and update with new score
    let users: users::ActiveModel = Users::find_by_id(data.login)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
        .map(Into::into)?;

    // user to update i don't know if its work
    users::ActiveModel {
        login: users.login,
        score: Set(data.score.to_owned()),
        profile_pic: users.profile_pic,
    }
    .update(db)
    .await
}