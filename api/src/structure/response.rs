use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize)]
pub struct LoginResponseStructure {
    pub message: String,
    pub success: bool,
    pub token : Option<String>
}

#[derive(Serialize)]
pub struct SignupResponseStructure {
    pub message: String,
    pub success: bool,
    pub user : Option<User>
}

#[derive(Serialize)]
pub enum JobType { SendEmail }

#[derive(Serialize)]
pub enum JobStatus { Pending , Completed, Failed }

#[derive(Serialize)]
pub struct JobResponseData {
    pub job_type : JobType,
    pub status : JobStatus,
    pub result : String,
    pub duration_ms : i32,
    pub attempts : i32
}


#[derive(Serialize)]
pub struct JobResponseStructure{
    pub message : String,
    pub success : bool,
    pub data : JobResponseData
}