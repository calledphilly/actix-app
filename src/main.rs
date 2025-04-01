use actix_cors;
use actix_session;
use actix_web::cookie::Key;
use dotenv;
mod utils;
use sqlx::postgres::PgPoolOptions;
use utils::services::{hello_handler, login_handler, logout_handler, users_handler};

#[actix_web::main] 
async fn main() -> std::io::Result<()> {    
    dotenv::dotenv().ok();
    let host = std::env::var("HOST").expect("Error occured while unwrapping of HOST");
    let host_cloned = host.clone();
    let port_actix_app = std::env::var("PORT_ACTIX_APP").expect("Error occured while unwrapping of PORT_ACTIX_APP");
    let port_yew_app = std::env::var("PORT_YEW_APP").expect("Error occured while unwrapping of PORT_YEW_APP");
    let postgres_url = std::env::var("POSTGRES_URL").expect("Error occured while unwrapping of POSTGRES_URL");
    let redis_url = std::env::var("REDIS_URL").expect("Error occured while unwrapping of REDIS_URL");
    let redis_store = actix_session::storage::RedisSessionStore::new(redis_url)
        .await
        .expect("Error occured while unwrapping of redis_store");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(postgres_url.as_str())
        .await
        .expect("Error occured while connecting to the postgres's database");
    let cookie_secret_key = Key::generate();
    
    println!("\nServer running on : http://{}:{}",host,port_actix_app);
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .wrap(actix_session::SessionMiddleware::new(
                redis_store.clone(),
                cookie_secret_key.clone(),
            ))
            .wrap(actix_identity::IdentityMiddleware::default())
            .wrap(actix_web::middleware::NormalizePath::default())
            .wrap(actix_cors::Cors::default()
                // .allow_any_origin()
                .allowed_origin(format!("http://{}:{}",host_cloned,port_yew_app).as_str())
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
    .bind((host, port_actix_app.parse::<u16>().unwrap()))?
    .run()
    .await
}
