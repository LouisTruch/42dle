use rocket::response::status::{self, Accepted, BadRequest};
use sea_orm::{DatabaseConnection, Set};
use sea_orm::*;
use crate::entities::{prelude::*, *};

pub async fn new_user(
    db: &DatabaseConnection,
    login: &String, profile_pic: &String
) -> Result<InsertResult<users::ActiveModel>, DbErr> {

    let record = users::ActiveModel {
        login: Set(login.to_owned()),
        profile_pic: Set(profile_pic.to_owned()),
        score: Set(Some(0)),
        ..Default::default()
    };

    Users::insert(record).exec(db).await
    //     Ok(_) => println!("User Created!"),
    //     Err(e) => { 
    //         println!("{}", e);
    //         return status::BadRequest(Some("User already in database!"));
    //     }
    // };
    // // .save(db)
    // // .await
    // status::Accepted(Some("New User Created!"))
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