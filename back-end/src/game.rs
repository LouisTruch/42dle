use rand::Rng;

pub async fn generate_target (){
    println!("INSIDE THIS FUNC");
    let token = "cf16c9c69086c8423a8c6814f93be457f607b6490ef12af5ec1e1eb8de425d4d";
    // let rng = rand::thread_rng();
    let client: reqwest::Client = reqwest::Client::new();
    let mut bearer: String = String::from("Bearer ").to_owned();
    bearer.push_str(&token);

    let campus_users = client.get("https://api.intra.42.fr/v2")
       .header("Authorization", bearer.as_str())
       .send()
       .await
       .expect("generate_token: Response from 42's api failed");
    println!("CAMPUS: {:?}", campus_users);
    // let index = rng.gen_range(0..campus_users.len());
    // let res = client.get("https://api.intra.42.fr/users/{index}")
    //     .header("Authorization", bearer.as_str())
    //     .send()
    //     .await
    //     .expect("get_user_data: Response from 42's api failed");
    // println!("User data: {:?}", res);
//         .json::<newType>()
//         .await
//         .expect("get_user_data: Parse the response from 42's api failed");
}