use crate::service::post_service::PostService;
use actix_web::{get, web, HttpResponse};
use std::sync::Arc; // เพิ่ม Arc

#[get("/posts")]
pub async fn get_posts(service: web::Data<Arc<PostService>>) -> HttpResponse {
    match service.get_all() {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
