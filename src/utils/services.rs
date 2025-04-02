use crate::utils::users::User;
use actix_identity::Identity;
use actix_web::{get, post, HttpMessage, HttpResponse};

pub mod users {
    use actix_web::{get, post, HttpResponse, Responder};
    use bcrypt::DEFAULT_COST;
    use crate::utils::users::User;

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
    pub async fn get_users(db_pool: actix_web::web::Data<sqlx::PgPool>) -> impl Responder {
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

    #[post("/users")]
    pub async fn create_user(form:actix_web::web::Form<User>,db_pool:actix_web::web::Data<sqlx::PgPool>) -> impl Responder{
        let data = form.into_inner();
        let hashed_password = bcrypt::hash(data.password.unwrap(), DEFAULT_COST);
        let response = sqlx::query(r#"INSERT INTO users (email, username, password, firstname, lastname) VALUES ($1, $2, $3, $4, $5)"#)
            .bind(&data.email.unwrap())
            .bind(&data.username.unwrap())
            .bind(&hashed_password.unwrap())
            .bind(&data.firstname.unwrap())
            .bind(&data.lastname.unwrap())
            .execute(db_pool.as_ref())
            .await;
        match response {
            Ok(v) => {
                println!("Creation user successful ! : {:?}",v);
                HttpResponse::Ok().body("Creation user successful !")
            },
            Err(e) => {
                eprintln!("Error occured while user creating : {}",e);
                HttpResponse::InternalServerError().body("Internal server error")
            } 
        }
    }
}

#[post("/login")]
pub async fn login(
    db_pool: actix_web::web::Data<sqlx::PgPool>,
    request: actix_web::HttpRequest,
    form: actix_web::web::Form<User>,
) -> impl actix_web::Responder {
    let data = form.into_inner();
    let db_user = sqlx::query_as::<_, User>(r#"SELECT * FROM users WHERE username = $1"#)
        .bind(&data.username)
        .fetch_one(db_pool.as_ref())
        .await;
    match db_user {
        Ok(db_user) => match bcrypt::verify(&data.password.expect("data hasn't password"), &db_user.password.expect("db_user hasn't password")) {
            Ok(true) => {
                match actix_identity::Identity::login(&request.extensions(), db_user.id.expect("db_user hasn't ID").to_string()) {
                    Ok(_) => actix_web::HttpResponse::Ok()
                        .body(format!("Welcom {} !", &data.username.expect("data hasn't password"))),
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
pub async fn logout(user: Identity) -> impl actix_web::Responder {
    user.logout();
    HttpResponse::Ok()
}

#[get("/hello")]
async fn hello(identity: Identity) -> impl actix_web::Responder {
    match identity.id() {
        Ok(v) => HttpResponse::Ok().body(format!("******\n{}\n******", v)),
        Err(e) => HttpResponse::Unauthorized().body(format!("******\n{}\n******", e)),
    }
}
