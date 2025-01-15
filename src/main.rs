use actix_web::{web, App, HttpServer, Responder};
use actix_cors::Cors;
use chrono::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct TimeResponse {
    dubai_time: String,
}

async fn get_dubai_time() -> impl Responder {
    // Dubai is at UTC+4
    let dubai_offset = FixedOffset::east(4 * 3600);
    let dubai_time = Utc::now().with_timezone(&dubai_offset);

    let response = TimeResponse {
        dubai_time: dubai_time.format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    web::Json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Use environment variables or default to localhost:8080
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
            .route("/api/dubai-time", web::get().to(get_dubai_time))
    })
    .bind(address)?
    .run()
    .await
}
