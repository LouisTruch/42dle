use crate::auth::User;

#[get("/", rank = 2)]
pub fn no_auth_index() -> &'static str {
    "Your are at home not log"
}

#[get("/")]
pub fn index(_user: User) -> &'static str {
    "Your are at home logged"
}