use actix_web::{web, App, HttpResponse, HttpServer};

use crate::routes::user::{user_log_in, user_sign_up};

pub mod routes;
pub mod structure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/auth")
                            .route("/user/log-in", web::post().to(user_log_in))
                            .route("/user/sign-up", web::post().to(user_sign_up))
                    )
                    .service(
                        web::scope("/test")
                            .route("/main", web::get().to(|| async { HttpResponse::Ok().body("User profile") }))
                    )
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
