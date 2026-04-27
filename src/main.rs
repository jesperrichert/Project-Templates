use actix_web::{cookie::time::macros::date, dev::AppConfig, web, App, HttpServer};
use postgres::Client;
use std::sync::Mutex;

pub mod database;
pub mod routes;

pub struct ApiConfig {
    postgres: Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_uri = std::env::var("REDIS_URI").expect("redis_uri not set");

    let apiConfig = ApiConfig {
        postgres= database::postgres::postgres()
    }
    
    let route_config = web::Data::new(RouteConfig {
        redis_client: Mutex::new(connect_redis(redis_uri)),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(route_config.clone())
            .service(routes::index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
