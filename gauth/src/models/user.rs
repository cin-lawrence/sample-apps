use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}
