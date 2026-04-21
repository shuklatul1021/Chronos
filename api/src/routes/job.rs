use std::sync::Mutex;

use actix_web::{Responder, web};
use redis_client::redis;
use store::connect::Store;

use crate::structure::{request::{JobRequestStructure}, response::{JobResponseData, JobResponseStructure, JobType, JobStatus}};

pub async fn job_route(
    req : web::Json<JobRequestStructure>,
    db_store: web::Data<Mutex<Store>> 
) -> Result<impl Responder, actix_web::Error> {
    let mutex_store = db_store.lock().unwrap();
    let updated_db = mutex_store.submit_job(req.job_title, 
                                            req.job_description, 
                                            req.job_type, 
                                            req.payload, 
                                            req.runs_at, 
                                            req.max_retries, 
                                            req.webhook_url, 
                                            req.user_id
                                        );

    Ok(web::Json(JobResponseStructure {
        message : "Job Submit Successfully".to_string(),
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