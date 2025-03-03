use std::fmt::Display;
use crate::hasher::PasswordHasher;

pub struct User {
    pub id: u32,
    pub name: String,
    pub password: String,
}

impl User {
    pub fn new(id: u32, name: &str, password: &str) -> Self {
        Self {
            id,
            name: String::from(name),
            password: String::from(password),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn set_password<H: PasswordHasher>(&mut self, password: &str, hasher: &H) {
        self.password = hasher.hash_password(password);
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "id:{}\nname:{}\npassword:{}\n", self.id, self.name, self.password)
    }
}
