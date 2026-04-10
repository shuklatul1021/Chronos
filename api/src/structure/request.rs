use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequestStructure {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignupRequestStructure {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub enum JobType { SendEmail, Notification, PdfGeneration }

#[derive(Deserialize)]
pub enum Priority { High , Low, Medium }

#[derive(Deserialize)]
pub struct JobRequestStructure{
    pub job_type : JobType,
    pub payload : String,
    pub priority : Priority,
    pub run_at : String,
    pub max_retries : i32,
    pub webhook_url : String
}


