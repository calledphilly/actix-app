use std::{process::Command, thread, time::Duration};
use actix_cors;
use actix_session;
use actix_web::cookie::Key;
use dotenv;
use sqlx::postgres::PgPoolOptions;
mod utils;
use utils::services::{hello, login, logout, users::{get_users, users_handler_legacy}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Command::new("docker-compose")
        .arg("up")
        .arg("-d")
        .status()
        .expect("Error occured while starting of Docker Compose");
 
    thread::sleep(Duration::from_millis(200));
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
    println!("Press Control(^)+C to stop Server\n");
    /* let server = */
    let server = actix_web::HttpServer::new(move || {
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
                // .allow_any_method()
            )
            .service(users_handler_legacy)
            .service(login)
            .service(logout)
            .service(actix_web::web::scope("/account")
                .service(hello)
            )
            .service(actix_web::web::scope("/api")
                .service(get_users)
            )
            .default_service(actix_web::web::route().method(actix_web::http::Method::GET))
    })
    .bind((host, port_actix_app.parse::<u16>().unwrap()))?
    .run();

    let server_handle = server.handle();
    
    actix_web::rt::spawn(async move {
        actix_web::rt::signal::ctrl_c().await.expect("Error occured while waiting SIGINT signal");
        println!(" : SIGINT received, closing in process...");
        server_handle.stop(true).await;
    });

    server.await?;

    Command::new("docker-compose")
        .arg("down")
        .status()
        .expect("Error occured while closing from Docker Compose");

    Ok(())
}
