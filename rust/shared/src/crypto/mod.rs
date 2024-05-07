use magic_crypt::{new_magic_crypt, MagicCryptTrait};

use crate::errors::BusinessError;

pub mod helpers;
pub mod otp;
pub mod auth;
pub mod jwt;
pub mod crypto;

pub fn decrypt(encrypted: &str, encryption_key: &str) -> Result<String, BusinessError> {
    let mc = new_magic_crypt!(encryption_key, 256);

    match mc.decrypt_base64_to_string(encrypted) {
        Ok(result) => Ok(result.to_owned()),
        Err(e) => Err(BusinessError::DecryptionError(format!("Unable to decrypt: {:?}", e))),
    }
}

pub fn encrypt(private_key: &str, encryption_key: &str) -> String {

    let mc = new_magic_crypt!(encryption_key, 256);
    let ref this = mc;
    this.encrypt_to_base64(private_key)
}
