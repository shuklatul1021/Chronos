use actix_web::{Responder, web};

use crate::structure::{request::{JobRequestStructure}, response::{JobResponseData, JobResponseStructure, JobType, JobStatus}};

pub async fn job_route(
    req : web::Json<JobRequestStructure>,
) -> Result<impl Responder>{

    Ok(web::Json(JobResponseStructure {
        message : "Job Submit Successfull".to_string(),
        success : true,
        data : JobResponseData {
            job_type : JobType::SendEmail,
            status : JobStatus::Completed,
            result : "Task Completed".to_string(),
            duration_ms : 43,
            attempts : 43
        }
    }))
}

pub async fn get_job() -> impl Responder {
    
    
}