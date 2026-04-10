use std::sync::{Arc, Mutex};

use actix_web::{web, HttpResponse, Responder};
use store::connect::Store;


use crate::structure::{request::{LoginRequestStructure, SignupRequestStructure}, response::SignupResponseStructure};

pub async fn user_log_in(
    req: web::Json<LoginRequestStructure>
) -> Result<impl Responder, actix_web::Error> {
    let email = &req.email;

    Ok(HttpResponse::Ok().body(format!("User with email {} logged in successfully", email)))
}

pub async fn user_sign_up(
    req: web::Json<SignupRequestStructure>, 
    db_store: web::Data<Arc<Mutex<Store>>>
) -> Result<impl Responder, actix_web::Error> {
    let working = db_store.lock().unwrap();

    Ok(web::Json(SignupResponseStructure {
        message: "User registered successfully".to_string(),
        success: true,
    }))
}
