use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, web};
use jsonwebtoken::{EncodingKey, Header, decode, encode};
use store::connect::Store;

use serde::{Deserialize, Serialize};

use crate::structure::{
    auth::UserAuthStructure,
    request::{LoginRequestStructure, SignupRequestStructure},
    response::{LoginResponseStructure, SignupResponseStructure, User},
};

pub async fn user_log_in(
    req: web::Json<LoginRequestStructure>,
    db_store: web::Data<Arc<Mutex<Store>>>,
) -> Result<impl Responder, actix_web::Error> {
    let lock = db_store.lock().unwrap();
    let log_in_user = lock.login_user(req.email, req.password);
    match log_in_user {
        Ok(is_logged_in) => {
            // Adding Sending Jwt
            let my_auth = UserAuthStructure {
                id: "cxcxcxc".to_string(),
                email: req.email.clone(),
                exp: 10000000000,
            };
            let token = encode(
                &Header::default(),
                &my_auth,
                &EncodingKey::from_secret("thisisonlyfotext".as_ref()),
            )
            .map_err(|_| actix_web::error::ErrorInternalServerError("Token encoding failed"))?;

            if is_logged_in {
                Ok(web::Json(LoginResponseStructure {
                    message: "User Logged In Successfully".to_string(),
                    success: true,
                    token : Some(token)
                }))
            } else {
                Ok(web::Json(LoginResponseStructure {
                    message: "Invalid email or password".to_string(),
                    success: false,
                    token : None
                }))
            }
        }
        Err(_) => Ok(web::Json(LoginResponseStructure {
            message: "An error occurred while logging in".to_string(),
            success: false,
            token : None
        })),
    }
}

pub async fn user_sign_up(
    req: web::Json<SignupRequestStructure>,
    db_store: web::Data<Arc<Mutex<Store>>>,
) -> Result<impl Responder, actix_web::Error> {
    let lock = db_store.lock().unwrap();
    let sign_up_user = lock.register_user(req.username, req.email, req.password);

    match sign_up_user {
        Ok(is_sign_up) => {
            if is_sign_up.email == req.email {
                Ok(web::Json(SignupResponseStructure {
                    message: "Registration Completed".to_string(),
                    success: true,
                    user: Some( User {
                        id: is_sign_up.id,
                        username: is_sign_up.username,
                        email: is_sign_up.email,
                        created_at: is_sign_up.created_at,
                        updated_at: is_sign_up.updated_at,
                    }),
                }))
            } else {
                Ok(web::Json(SignupResponseStructure {
                    message: "Incorrect Credential".to_string(),
                    success: false,
                    user: None,
                }))
            }
        }
        Err(_) => Ok(web::Json(SignupResponseStructure {
            message: "Internal Server Error".to_string(),
            success: false,
            user: None,
        })),
    }
}
