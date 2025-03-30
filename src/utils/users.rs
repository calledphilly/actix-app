use serde::Deserialize;

pub trait GetData {
    fn get_username(&self) -> String;
    fn get_password(&self) -> String;
    fn get_id(&self) -> String;
}
#[derive(Clone, Deserialize)]
pub struct User {
    id: String,
    username: String,
    firstname: String,
    lastname: String,
    password: String,
    born_at: String,
    get_married: bool,
    city: String
}
impl GetData for User {
    fn get_username(&self) -> String {
        self.username.clone()
    }
    fn get_password(&self) -> String {
        self.password.clone()
    }
    fn get_id(&self) -> String {
        self.id.clone()
    }
}
