mod user;
mod hasher;
mod user_service;

use user::User;
use hasher::{PlainHasher, Sha256Hasher};
use user_service::UserService;

fn main() {
    let mut user = User::new(1, "Name", "Password");

    let plain_hasher = PlainHasher;
    let sha256_hasher = Sha256Hasher;

    user.set_password("my_password", &plain_hasher);
    println!("Plain Hash: {}", user);

    user.set_name("new name");
    user.set_password("my_password", &sha256_hasher);
    println!("SHA-256 Hash: {}", user);

    let is_valid_password = UserService::validate_password(&user, "my_password", &sha256_hasher);
    println!("Password is valid: {}", is_valid_password);
}

#[cfg(test)] 
mod tests {
    use crate::user::User;
    use crate::user_service::UserService;
    use crate::hasher::Sha256Hasher;

    #[test]
    fn test_validate_user_with_valid_password() {
        let mut user = User::new(1, "Alice", ""); 
        let sha256_hasher = Sha256Hasher;
        
        user.set_password("Alice123", &sha256_hasher);

        // ตรวจสอบการ validate password
        assert!(UserService::validate_password(&user, "Alice123", &sha256_hasher));
    }
}
