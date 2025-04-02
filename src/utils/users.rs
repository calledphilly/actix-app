use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::{self, prelude::FromRow};

pub trait GetData {
    fn _get_email(&self) -> Option<String>;
    fn get_username(&self) -> Option<String>;
    fn get_password(&self) -> Option<String>;
    fn get_id(&self) -> Option<String>;
}
#[derive(Clone, Deserialize, Serialize, FromRow, Validate)]
pub struct User {
    #[garde(alphanumeric)]
    id: Option<String>,
    #[garde(email)]
    email: Option<String>,
    #[garde(alphanumeric)]
    username: Option<String>,
    #[garde(length(min = 8))]
    password: Option<String>,
    #[garde(ascii)]
    firstname: Option<String>,
    #[garde(ascii)]
    lastname: Option<String>,
    #[garde(alphanumeric)]
    year: Option<String>,
    #[garde(ascii)]
    city: Option<String>,
}
impl GetData for User {
    fn _get_email(&self) -> Option<String> {
        self.email.clone()
    }
    fn get_username(&self) -> Option<String> {
        self.username.clone()
    }
    fn get_password(&self) -> Option<String> {
        self.password.clone()
    }
    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }
}
