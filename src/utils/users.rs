use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub trait GetData {
    fn _get_email(&self) -> String;
    fn get_username(&self) -> String;
    fn get_password(&self) -> String;
    fn get_id(&self) -> String;
}
#[derive(Clone, Deserialize, Serialize, FromRow)]
pub struct User {
    id: String,
    email: String,
    username: String,
    hashed_password: String,
    firstname: String,
    lastname: String,
    year: String,
    born_at: String,
    get_married: bool,
    city: String,
}
impl GetData for User {
    fn _get_email(&self) -> String {
        self.email.clone()
    }
    fn get_username(&self) -> String {
        self.username.clone()
    }
    fn get_password(&self) -> String {
        self.hashed_password.clone()
    }
    fn get_id(&self) -> String {
        self.id.clone()
    }
}
