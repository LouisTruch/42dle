use std::env;

#[get("/token/<code>")]
pub async fn generate_token(code: &str) -> String {
    let client: reqwest::Client = reqwest::Client::new();
    let res: Result<reqwest::Response, reqwest::Error> = client.post("https://api.intra.42.fr/oauth/token")
        .header("grant_type", "authorization_code")
        .header("client_id", env::var("CLIENT_ID").unwrap())
        .header("client_secret", env::var("CLIENT_SECRET").unwrap())
        .header("code", code)
        .send()
        .await;

    match res {
        Ok(_res) =>{
            format!("generateToken: {}", _res.text().await.expect("failed"))
        }
        Err(err) =>{
            format!("Error in generateToken: {}", err)
        }
    }
}

#[get("/users")]
pub async fn get_all_users() -> String {
    let token_plus_tard: String = String::from("Bon").to_owned();
    let mut bearer: String = String::from("Bearer ").to_owned();
    let client = reqwest::Client::new();

    bearer.push_str(&token_plus_tard);
    let res = client.get("https://api.intra.42.fr/v2/users")
        .header("Authorization", bearer.as_str())
        .send()
        .await;

    match res {
        Ok(_res) =>{
            format!("getAllUsers: {}", _res.text().await.expect("failed"))
        }
        Err(err) =>{
            format!("Error in getAllUsers: {}", err)
        }
    }
}