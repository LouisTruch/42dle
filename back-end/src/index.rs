
#[get("/", rank = 2)]
pub fn no_auth_index() -> &'static str {
    "Your are at home not log"
}

#[get("/")]
pub fn index() -> &'static str {
    "Your are at home logged"
}