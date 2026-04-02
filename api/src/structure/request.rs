use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequestStructure {
    pub email: String,
    pub passowrd: String,
}

#[derive(Deserialize)]
pub struct SignupRequestStructure {
    pub username: String,
    pub email: String,
    pub passowrd: String,
}
