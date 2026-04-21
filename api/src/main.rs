use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpResponse, HttpServer};
use redis_client::redis::RedisClient;
use store::connect::Store;
use crate::routes::{job::{job_route}, user::{user_log_in, user_sign_up}};

pub mod routes;
pub mod structure;
pub mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_store = web::Data::new(Mutex::new(Store::default().unwrap()));

    HttpServer::new(move || {
        App::new()
            .app_data(db_store.clone())
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/auth")
                            .route("/user/log-in", web::post().to(user_log_in))
                            .route("/user/sign-up", web::post().to(user_sign_up))
                    )
                    .service(
                        web::scope("/job")
                            .route("/submit-job", web::post().to(job_route))
                            .route("/get-job", web::get().to(get_job))
                    )
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
