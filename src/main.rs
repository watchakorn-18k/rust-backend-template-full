use crate::config::db::establish_connection_pool;
use crate::repository::post_repo::PostRepository;
use crate::repository::user_repo::UserRepository;
use crate::service::post_service::PostService;
use crate::service::user_service::UserService;
use actix_web::{middleware, web, App, HttpServer};
use env_logger;
use log::info;
use std::env;
use std::sync::Arc;

mod config;
mod dto;
mod handler;
mod models;
mod repository;
mod schema;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let port = env::var("PORT").unwrap_or("1323".to_string());
    info!("Starting server at http://127.0.0.1:{}", port);

    // DbPool
    let pool = establish_connection_pool();

    // Repository
    let user_repo = UserRepository::new(pool.clone());
    let post_repo = PostRepository::new(pool.clone());

    // Services
    let user_service = Arc::new(UserService::new(user_repo));
    let post_service = Arc::new(PostService::new(post_repo));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::new("%a '%r' %s %Dms"))
            .app_data(web::Data::new(user_service.clone()))
            .app_data(web::Data::new(post_service.clone()))
            .service(handler::health_check::health_check)
            .service(handler::user_handler::get_users)
            .service(handler::post_handler::get_posts)
            // --- WebSocket route ---
            .route("/ws", web::get().to(handler::ws_handler::ws_upgrade))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
