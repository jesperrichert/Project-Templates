use crate::ApiConfig;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CloudGETRequest {
    data: String,
}

#[derive(Serialize, Deserialize)]
pub struct GETResponse {
    message: String,
    success: bool,
}

#[get("/")]
pub async fn route( 
    // data: web::Query<CloudGETRequest>,
    config: web::Data<ApiConfig>) -> impl Responder {
    HttpResponse::Ok().json(GETResponse {
        message: "Hello World".to_string(),
        success: true,
    })
}
