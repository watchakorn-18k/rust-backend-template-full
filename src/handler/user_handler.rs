use crate::service::user_service::UserService;
use actix_web::{web, HttpResponse};
use std::sync::Arc; // เพิ่ม Arc

pub async fn get_users(service: web::Data<Arc<UserService>>) -> HttpResponse {
    match service.get_all() {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
