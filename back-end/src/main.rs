use std::env;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/token/<code>")]
async fn token(code: &str) -> String {
    // format!("Hello, new user: {}", name)
    let client = reqwest::Client::new();
    let res = client.post("https://api.intra.42.fr/oauth/token")
    .header("grant_type", "authorization_code")
    .header("client_id", env::var("CLIENT_ID").unwrap())
    .header("client_secret", env::var("CLIENT_SECRET").unwrap())
    .header("code", code)
    // .header("redirect_uri", uuid)
    .send()
    .await?;
    "Hello, your code is ".to_string() + code
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, token])
}