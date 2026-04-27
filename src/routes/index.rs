use crate::types::route::RouteConfig;
use actix_web::cookie::time::macros::date;
use actix_web::{HttpRequest, HttpResponse, Responder, error, get, post, web};
use redis::TypedCommands;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CloudGETRequest {
    key: String,
}

#[derive(Serialize, Deserialize)]
pub struct CloudGETResponse {
    message: String,
    data: String,
    success: bool,
}

#[get("/")]
async fn route(
    data: web::Query<CloudGETRequest>,
    config: web::Data<RouteConfig>,
) -> impl Responder {
    HttpResponse::Ok().json(CloudGETResponse {
        message: "Export from the Cloud...".to_string(),
        data: exists.unwrap(),
        success: true,
    })
}
