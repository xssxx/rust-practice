use sha2::{Sha256, Digest};

pub trait PasswordHasher {
    fn hash_password(&self, password: &str) -> String;
}

pub struct PlainHasher;
impl PasswordHasher for PlainHasher {
    fn hash_password(&self, password: &str) -> String {
        password.to_string()
    }
}

pub struct Sha256Hasher;
impl PasswordHasher for Sha256Hasher {
    fn hash_password(&self, password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password);
        format!("{:x}", hasher.finalize())
    }
}
