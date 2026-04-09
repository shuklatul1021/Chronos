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


#[derive(Serialize)]
pub enum JobType { SendEmail }

#[derive(Serialize)]
pub enum JobStatus { Pending , Completed, Failed }

#[derive(Serialize)]
pub struct JobResponseData{
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