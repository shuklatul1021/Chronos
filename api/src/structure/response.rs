use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponseStructure {
    pub message: String,
    pub success: bool,
}

#[derive(Serialize)]
pub struct SignupResponseStructure {
    pub message: String,
    pub success: bool,
}
