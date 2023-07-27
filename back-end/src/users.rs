use sea_orm::{DatabaseConnection, Set};
use sea_orm::EntityTrait;
use rocket::response::status;
use rocket::State;
use crate::entities::{prelude::*, *};

#[post("/new/<login>")]
pub async fn new_user(db_conn: &State<DatabaseConnection>, login: &str) -> status::Accepted<&'static str> {
    let db_conn = db_conn as &DatabaseConnection;

    let user = users::ActiveModel {
        login: Set(login.to_owned()),
        score: Set(Some(0)),
        ..Default::default()
    };

    let _result = Users::insert(user.clone()).exec(db_conn).await;

    status::Accepted(Some("User Created"))
}