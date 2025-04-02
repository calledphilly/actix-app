use crate::utils::users::{self, GetData, User};
use actix_identity::Identity;
use actix_web::{get, post, HttpMessage, HttpResponse};

#[get("/users_legacy")]
pub async fn users_handler_legacy() -> impl actix_web::Responder {
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

#[get("/users")]
pub async fn users_handler(db_pool: actix_web::web::Data<sqlx::PgPool>) -> impl actix_web::Responder {
    let users = sqlx::query_as::<_, User>(r#"SELECT * FROM users"#)
        .fetch_all(db_pool.as_ref())
        .await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Error occured while fetching users from database : {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

#[post("/login")]
pub async fn login_handler(
    db_pool: actix_web::web::Data<sqlx::PgPool>,
    request: actix_web::HttpRequest,
    form: actix_web::web::Form<users::User>,
) -> impl actix_web::Responder {
    let data = form.into_inner();
    let db_user = sqlx::query_as::<_, User>(r#"SELECT * FROM users WHERE username = $1"#)
        .bind(data.get_username())
        .fetch_one(db_pool.as_ref())
        .await;
    match db_user {
        Ok(db_user) => match bcrypt::verify(data.get_password().expect("data don't has password"), &db_user.get_password().expect("db_user don't has password")) {
            Ok(true) => {
                match actix_identity::Identity::login(&request.extensions(), db_user.get_id().expect("db_user don't has id")) {
                    Ok(_) => actix_web::HttpResponse::Ok()
                        .body(format!("Welcom {} !", data.get_username().expect("data don't has password"))),
                    Err(e) => {
                        eprintln!("Error logging in: {}", e);
                        actix_web::HttpResponse::InternalServerError().body("Internal server error")
                    }
                }
            }
            Ok(false) => actix_web::HttpResponse::Unauthorized().body("Identifiants don't match"),
            Err(e) => {
                eprintln!("Error occured while processing from bcrypt : {}", e);
                actix_web::HttpResponse::InternalServerError().body("Internal server error")
            }
        },
        Err(sqlx::Error::RowNotFound) => actix_web::HttpResponse::Unauthorized().body("Identifiants don't match"),
        Err(e) => {
            eprintln!("Error occured while fetching of db_user : {}", e);
            actix_web::HttpResponse::InternalServerError().body("Internal server error")
        }
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
        Ok(v) => HttpResponse::Ok().body(format!("******\n{}\n******", v)),
        Err(e) => HttpResponse::Unauthorized().body(format!("******\n{}\n******", e)),
    }
}
