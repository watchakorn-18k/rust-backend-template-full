use crate::dto::health_check::HealthResponse;
use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
    })
}
