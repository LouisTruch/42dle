use rocket::request::{FromRequest, Request};
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::time::{Duration, OffsetDateTime};
use serde::{Deserialize, Serialize};
use rocket::State;
use rocket::request::*;
use rand::Rng;

pub fn generate_target (){
/*
   let acces_token = token from db's admin
*/
    let mut rng = rand::thread_rng();
    let client: reqwest::Client = reqwest::Client::new();
    let mut bearer: String = String::from("Bearer ").to_owned();
    bearer.push_str(&token);

    let campus_users = client.get("https://api.intra.42.fr/v2/campus_users")
       .header("Authorization","Bearer YOUR_ACCESS_TOKEN")
       .send()
       .await
       .expect("generate_token: Response from 42's api failed");

    let index = rng.gen_range(0..campus_users.len());
    let res = client.get("https://api.intra.42.fr/users/{index}")
        .header("Authorization", bearer.as_str())
        .send()
        .await
        .expect("get_user_data: Response from 42's api failed");
    println!("User data: {:?}", res);
//         .json::<newType>()
//         .await
//         .expect("get_user_data: Parse the response from 42's api failed");
}