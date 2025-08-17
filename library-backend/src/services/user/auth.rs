use crate::error::LibraryError;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;

pub struct UserAuth;

impl UserAuth {
    pub fn hash_password(password: &str) -> Result<String, LibraryError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| LibraryError::PasswordHashError)?
            .to_string();
        
        Ok(password_hash)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, LibraryError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| LibraryError::PasswordHashError)?;
        let argon2 = Argon2::default();
        
        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}