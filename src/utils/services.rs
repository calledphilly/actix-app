use crate::utils::users::{self, GetData, User};
use actix_identity::Identity;
use actix_web::{get, post, HttpMessage, HttpResponse};
// use sqlx;
#[get("/users")]
pub async fn users_handler() -> impl actix_web::Responder {
    let user1 = serde_json::json!({
        "id":1,
        "firstname":"Yannis",
        "lastname":"Bikouta",
        "year":"25",
        "city":"Limay",
        "getMarried":true,
        "bornAt":"Nîmes",
    });
    let user2 = serde_json::json!({
        "id":2,
        "firstname":"Amélie",
        "lastname":"Bikouta",
        "year":"25",
        "city":"Limay",
        "getMarried":true,
        "bornAt":"Mantes-la-Ville",
    });
    let data = serde_json::json!({
        "data":{
            "users":[user1, user2],
            "cities":[{"empty":"empty"},{"empty":"empty"}]
        }
    });
    actix_web::HttpResponse::Ok().json(data)
}

#[get("/users2")]
async fn users_handler2(db_pool: actix_web::web::Data<sqlx::PgPool>) -> HttpResponse {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(db_pool.as_ref())
        .await
        .unwrap_or_default();
    HttpResponse::Ok().json(users)
}

#[post("/login")]
pub async fn login_handler(
    request: actix_web::HttpRequest,
    form: actix_web::web::Form<users::User>,
) -> impl actix_web::Responder {
    let user = form.into_inner();
    if user.get_username() == "calledphilly" && user.get_password() == "azerty" {
        match actix_identity::Identity::login(&request.extensions(), user.get_id()) {
            Ok(_) => {
                actix_web::HttpResponse::Ok().body(format!("Bienvenue {} !", user.get_username()))
            }
            Err(e) => actix_web::HttpResponse::InternalServerError()
                .body(format!("Error logging in: {}", e)),
        }
    } else {
        actix_web::HttpResponse::Unauthorized().body("Identifiants invalides")
    }
}

#[post("/logout")]
pub async fn logout_handler(user: Identity) -> impl actix_web::Responder {
    user.logout();
    HttpResponse::Ok()
}

#[get("/hello")]
async fn hello_handler(identity: Identity) -> impl actix_web::Responder {
    match identity.id() {
        Ok(v) => HttpResponse::Ok().body(format!("******\n{}\n******",v)),
        Err(e) => HttpResponse::Unauthorized().body(format!("******\n{}\n******",e)),
    }
}
