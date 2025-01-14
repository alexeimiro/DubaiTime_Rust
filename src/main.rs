use actix_web::{web, App, HttpServer, Responder};
use chrono::prelude::*;
use serde::Serialize;
use std::sync::Arc;

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
    let app_data = Arc::new(());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .route("/api/dubai-time", web::get().to(get_dubai_time))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
