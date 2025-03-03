use crate::user::User;
use crate::hasher::PasswordHasher;

pub struct UserService;

impl UserService {
    pub fn validate_password<H: PasswordHasher>(user: &User, password: &str, hasher: &H) -> bool {
        let hashed_password = hasher.hash_password(password);
        user.password == hashed_password
    }
}

