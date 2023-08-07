use sea_orm::*;
use crate::entities::{prelude::*, *};


pub async fn new_user(
    db: &DatabaseConnection,
    login: &String, 
    profile_pic: &String
    ) -> Result<InsertResult<users::ActiveModel>, DbErr> {  

    // match Users::find_by_id(login).one(db).await {
    //     Ok(user) => return user,
    //     Err(_) => {} 
    // }
    
    // Create a record to add in users table
    let record = users::ActiveModel {
        login: Set(login.to_owned()),
        profile_pic: Set(profile_pic.to_owned()),
        r#try: Set([].to_vec()),
        ..Default::default()
    };

    // Insert in users tables and return the Result
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

    let game: Option<game::Model> = Game::find_by_id(1).one(db).await?;
    let mut game: game::ActiveModel = game.unwrap().into();
    // Check if try is equal to login of the day
    let find_login: String = game.login_to_find.unwrap().into();
    if new_try == &find_login {

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


pub async fn get_leaderboard(
    db: &DatabaseConnection,
) -> Result<Vec<users::Model>, DbErr> {

    Users::find()
        .order_by_desc(users::Column::Score)
        .all(db)
        .await
}


pub async fn new_day(
    db: &DatabaseConnection,
    new_login: &String,
    new_profile_pic: &String,
) -> Result<InsertResult<game::ActiveModel>, DbErr> {

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
    let new_day = game::ActiveModel {
        login_to_find: Set(new_login.to_owned()),
        profile_pic: Set(new_profile_pic.to_owned()),
        ..Default::default()
    };

    // Insert in game tables and return the Result
    Game::insert(new_day).exec(db).await
}