use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FullName {
    firstname: String,
    lastname: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    id: usize,
    email: String,
    username: String,
    password: String,
    name: FullName,
    phone: String,
}
