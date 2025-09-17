pub struct FullName {
    firstname: String,
    lastname: String,
}

pub struct User {
    id: usize,
    email: String,
    username: String,
    password: String,
    name: FullName,
    phone: String,
}
