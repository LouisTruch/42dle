use std::env;
use reqwest::Response;
use serde::Deserialize;
use rocket::tokio::time::sleep;
use std::time::Duration;

use crate::auth::Situation;

#[derive(Deserialize)]
pub struct ImageData {
    pub versions: Option<ImageVersions>,
}

#[derive(Deserialize)]
pub struct ImageVersions {
    pub medium: Option<String>,
}

#[derive(Deserialize)]
pub struct CursusLevel {
    pub level: f64,
}

#[derive(Deserialize)]
struct ApiData {
    login: String,
    image: Option<ImageData>,
    cursus_users: Vec<CursusLevel>,
}

#[derive(Deserialize)]
pub struct ApiToken {
    access_token: String,
}

pub async fn get_user_data(token: String) -> (String, String, String) {
    // Prepare the "Authorization" header by appending the token to "Bearer ".
    let mut bearer: String = String::from("Bearer ").to_owned();
    bearer.push_str(&token);
    println!("{bearer}");
    // Send a GET request to 42's api with the "Authorization" header.
    // wait the response and parse it into the 'ApiData' struct.
    let client = reqwest::Client::new();
    let res = client.get("https://api.intra.42.fr/v2/me")
        .header("Authorization", bearer.as_str())
        .send()
        .await
        .expect("get_user_data: Response from 42's api failed")
        .json::<ApiData>()
        .await
        .expect("get_user_data: Parse the response from 42's api failed");

    // From this struct it will Extract login & image url & cursus.
    let situation = if res.cursus_users.len() > 1 {
        Situation::Stud.to_string()
    } else { Situation::Pool.to_string() };
    // Return them as a tuple.
    return (res.login, res.image
        .expect("get_user_data: user without profil pic")
        .versions
        .expect("get_user_data: user without profil pic")
        .medium
        .expect("get_user_data: user without profil pic")
    , situation
    );
}

pub async fn generate_token(code: &str, ) -> String{
    // Get informations in .env file to generate request's body for 42's api
    let client_id: String =  env::var("CLIENT_ID").expect("CLIENT_ID not found in .env");
    let client_secret: String =  env::var("CLIENT_SECRET").expect("CLIENT_SECRET not found in .env");
    let data = [("grant_type", "authorization_code"),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
        ("code", &code), 
        ("redirect_uri", "http://localhost:5173/auth"),
    ];

    // Send a request to 42's API with a request body.
    // The API will return a response in JSON format.
    // This JSON response will be parsed to extract access_token and return it as a String.
    let client: reqwest::Client = reqwest::Client::new();
    let access_token = client.post("https://api.intra.42.fr/oauth/token")
        .header("Content-Type","application/x-www-form-urlencoded")
        .form(&data)
        .send()
        .await
        .expect("generate_token: Response from 42's api failed")
        .json::<ApiToken>()
        .await
        .expect("generate_token: Parse the response from 42's api failed")
        .access_token;
    access_token
}

// Use to deserialize campus users from 42's api
#[derive(Deserialize)]
pub struct CampusStudent {
    pub login: String,
    pub first_name: String,
    pub last_name: String,
    pub image: Option<ImageData>,
    #[serde(rename = "alumni?")]
    alumni: Option<bool>,
    #[serde(rename = "active?")]
    active: Option<bool>,
}


// return the number of pages based on number of users in a campus
fn get_numbers_pages(campus_users: &Response) -> i32 {
    // get the number of users in a campus
    let nb_users = campus_users.headers()
        .get("X-Total")
        .expect("get_users_campus token: get X-Total error");

    // Divide the total number of users on a campus by the number of users per single request.
    // This will provide the total number of requests required to collect every user on the campus.
    (nb_users
    .to_str()
    .expect("get_users_campus: Can't convert the number of user from request into an int")
    .parse::<f32>()
    .unwrap() / 100.0).ceil() as i32

}

// Collect every user on a campus
// Except user who havn't profil pic, inactive users, or alumni users
pub async fn get_users_campus (token: String) -> Vec<CampusStudent>{
    let mut users: Vec<CampusStudent> = Vec::new();
    let client: reqwest::Client = reqwest::Client::new();
    let mut bearer: String = String::from("Bearer ").to_owned();
    bearer.push_str(&token);

    // Iterate while there are remaining users to get
    let mut nb_pages= 2;
    let mut i = 1;
    while i <= nb_pages{
        let mut url: String = String::from("https://api.intra.42.fr/v2/campus/31/users?per_page=100&page=").to_owned();
        url.push_str(&i.to_string());
        let campus_users = client.get(url)
            .header("Authorization", bearer.as_str())
            .send()
            .await
            .expect("get_users_campus: Response from 42's api failed");

        // In the first request, parse X-Total to determine the number of request to do
        nb_pages = if i == 1 { get_numbers_pages(&campus_users) } else { nb_pages };

        // parse response as Json struct caontaining user data
        let new_page = campus_users
            .json::<Vec<CampusStudent>>()
            .await
            .expect("get_users_campus: Parse the response from 42's api failed");
        users.extend(new_page);
        // sleep between earh request to avoid overflowing 42's api
        sleep(Duration::from_millis(600)).await;
        i += 1;
    }

    // remove inactive or alumni users or users without profil pic
    let mut i: usize = 0;
    while i < users.len(){
        
        if verify_user(&mut users, i) == false{
            users.remove(i);
            continue;
        }
        i = i + 1;
    }
    users
}

// for each user: verify if he has a profil pic, isn't alunmi & if he is active
fn verify_user(users: &mut Vec<CampusStudent>, i: usize) -> bool{
    match &users[i].image {
        Some(img_data) => {
            match &img_data.versions {
                Some(version) => {
                    match &version.medium {
                        Some(_img) => {},
                        None => { 
                            return false;
                        }
                    }
                },
                None => { 
                    return false;
                }
            }
        },
        None => { 
            return false;
        }
    };
    let alumni = match users[i].alumni {
        Some(val) => {val},
        None => {true}
    };
    let active = match users[i].active {
        Some(val) => {val},
        None => {false}
    };
    if alumni == true || active == false 
        || users[i].last_name == "Angoule" || users[i].last_name == "Angouleme"{
        return false;
    }
    true
}