use std::env;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/token/<code>")]
async fn token(code: &str) -> String {
    let client = reqwest::Client::new();
    let res = client.post("https://api.intra.42.fr/oauth/token")
        .header("grant_type", "authorization_code")
        .header("client_id", env::var("CLIENT_ID").unwrap())
        .header("client_secret", env::var("CLIENT_SECRET").unwrap())
        .header("code", code)
        .send()
        .await;

    match res {
        Ok(_res) =>{
            // print!("Ok from match: {}", _res.text().await.expect("failed"));
            format!("Ok from match: {}", _res.text().await.expect("failed"))
        }
        Err(err) =>{
            // print!("Err from match: {}", err);
            format!("Err from match: {}", err)
        }
    }
    // .header("redirect_uri", uuid)
    // "Hello, your code is ".to_string() + code
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, token])
}