use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::{self, prelude::FromRow, Decode};

#[derive(Clone, Deserialize, Serialize, FromRow, Validate, Decode)]
pub struct User {
    #[garde(custom(validate_id))]
    pub id: Option<i32>,
    #[garde(email)]
    pub email: Option<String>,
    #[garde(alphanumeric)]
    pub username: Option<String>,
    #[garde(length(min = 8))]
    pub password: Option<String>,
    #[garde(ascii)]
    pub firstname: Option<String>,
    #[garde(ascii)]
    pub lastname: Option<String>,
    #[garde(alphanumeric)]
    pub year: Option<String>,
    #[garde(ascii)]
    pub city: Option<String>,
}
fn validate_id(id: &Option<i32>, _ctx: &()) -> garde::Result {
    match *id  {
        Some(id) => match id {
            0 => Err(garde::Error::new("ID must be greater than 0")),
            v if v > 1_000_000 => Err(garde::Error::new("ID must be less than 1,000,000")),
            _ => Ok(())
        },
        None => Ok(())
    }
}
