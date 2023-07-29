use sea_orm::*;
use crate::entities::{prelude::*, *};

pub async fn new_user(
    db: &DatabaseConnection,
    login: &String, 
    profile_pic: &String
    ) -> Result<InsertResult<users::ActiveModel>, DbErr> {  

    // Create a record to add in db
    let record = users::ActiveModel {
        login: Set(login.to_owned()),
        profile_pic: Set(profile_pic.to_owned()),
        r#try: Set([].to_vec()),
        ..Default::default()
    };

    // Insert in db and return the Result
    Users::insert(record).exec(db).await
}

pub async fn update_try_by_login(
    db: &DatabaseConnection,
    login: &String,
    new_try: &String
    ) -> Result<users::Model, DbErr> {

    // Find users in db with login ( primary key ) and update with new try
    let users: Option<users::Model> = Users::find_by_id(login).one(db).await?;
    let mut users: users::ActiveModel = users.unwrap().into();
    let mut new_vec: Vec<String> = users.r#try.unwrap().into();
    new_vec.push(new_try.to_string());
    users.r#try = Set(new_vec);

    let game: Option<game::Model> = Game::find_by_id(1).one(db).await?;
    let mut game: game::ActiveModel = game.unwrap().into();
    // Check if try is equal to login of the day
    let find_login: String = game.login_to_find.unwrap().into();
    if new_try == &find_login {
        users.win = Set(true);
    }
    game.login_to_find = Set(find_login);

    game.update(db).await?;
    users.update(db).await
}

pub async fn get_leaderboard(
    db: &DatabaseConnection,
) -> Result<Vec<users::Model>, DbErr> {

    Users::find()
        .order_by_desc(users::Column::Score)
        .all(db)
        .await
}