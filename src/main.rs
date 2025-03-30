use actix_cors;
use actix_session;
use dotenv;
mod utils;
use utils::services::{hello_handler, login_handler, logout_handler, users_handler};

#[actix_web::main] 
async fn main() -> std::io::Result<()> {    
    dotenv::dotenv().ok();
    let host = std::env::var("HOST").unwrap();
    let port = std::env::var("PORT").unwrap();
    let cookie_secret_key = actix_web::cookie::Key::generate();
    let redis_store = actix_session::storage::RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(actix_session::SessionMiddleware::new(
                redis_store.clone(),
                cookie_secret_key.clone(),
            ))
            .wrap(actix_identity::IdentityMiddleware::default())
            .wrap(actix_web::middleware::NormalizePath::default())
            .wrap(actix_cors::Cors::default()
                .allowed_origin("http://127.0.0.1:8000")
                .allow_any_header()
                .allowed_methods([actix_web::http::Method::GET])
            )
            .service(logout_handler)
            .service(login_handler)
            .service(actix_web::web::scope("/account")
                .service(hello_handler)
            )
            .service(actix_web::web::scope("/api")
                .service(users_handler)
            )
            .default_service(actix_web::web::route().method(actix_web::http::Method::GET))
    })
    .bind((host, port.parse::<u16>().unwrap()))?
    .run()
    .await
}
