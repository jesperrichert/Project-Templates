use actix_web::{web, App, HttpServer};
use redis::Connection;
use std::sync::Mutex;

pub mod database;
pub mod routes;

pub struct ApiConfig {
    pub postgres: tokio_postgres::Client,
    pub redis_client: Mutex<Connection>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_uri = std::env::var("REDIS_URI").expect("redis_uri not set");

    let api_config = web::Data::new(ApiConfig {
        postgres: database::postgres::postgres().await.unwrap(),
        redis_client: Mutex::new(database::redis::redis(redis_uri)),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(api_config.clone())
            .service(routes::index::route)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
