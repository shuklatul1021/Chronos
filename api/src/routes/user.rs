use actix_web::{web, HttpResponse, Responder};

use crate::structure::request::{LoginRequestStructure, SignupRequestStructure};

pub async fn user_log_in(req: web::Json<LoginRequestStructure>) -> impl Responder {
    let email = &req.email;

    HttpResponse::Ok().body(format!("User with email {} logged in successfully", email))
}

pub async fn user_sign_up(req: web::Json<SignupRequestStructure>) -> impl Responder {
    let email = req.email;

    HttpResponse::Ok().body(req_body)
}
