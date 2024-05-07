use k256::{SecretKey, PublicKey};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use k256::ecdsa::{SigningKey, Signature, VerifyingKey};
use k256::ecdsa::signature::{Signer, Verifier};
use base64::{Engine as _, engine::general_purpose::URL_SAFE as base64};

use crate::errors::BusinessError;

use super::helpers::{get_seed, hash256, gen_random_bytes};

pub fn sign_msg(message: &String, secret: &String, password: &String) -> Result<String, BusinessError> {
    let secret_vec = secret.as_bytes().to_vec();
    let seed: Vec<u8> = get_seed(&secret_vec, &password);
    let secret_key: SecretKey = match SecretKey::from_slice(&seed[0..32]) {
        Ok(secret_key) => secret_key,
        Err(e) => return Err(BusinessError::DecryptionError(format!("Unable to get secret key {}",e))), 
    };

    let signing_key = SigningKey::from(secret_key);
    let msg = hash256(&message.as_bytes().to_vec());

    let signature: Signature = signing_key.sign(&msg);
    let sig = signature.to_vec();

    Ok(base64.encode(sig))
}

pub fn verify_msg(message: &String, sig_base64: &String, public_key_base64: &String) -> Result<bool, BusinessError> {
    let sig = match base64.decode(sig_base64) {
        Ok(sig) => sig,
        Err(e) => return Err(BusinessError::DecryptionError(format!("Unable to decode signature: {}", e))),
    };
    let signature = match Signature::try_from(sig.as_slice()) {
        Ok(sig) => sig,
        Err(e) => return Err(BusinessError::DecryptionError(format!("Unable to convert signature: {}", e))),
    };

    let pub_key = match base64.decode(public_key_base64) {
        Ok(pub_key) => pub_key,
        Err(e) =>  return Err(BusinessError::DecryptionError(format!("Unable to decode public key: {}", e))),
    };

    let public_key = match VerifyingKey::from_sec1_bytes(&pub_key) {
        Ok(public_key) => public_key,
        Err(e) => return Err(BusinessError::DecryptionError(format!("Unable to convert public key: {}", e))),
    };

    let msg = hash256(&message.as_bytes().to_vec());

    match public_key.verify(&msg, &signature) {
        Ok(_) => Ok(true),
        Err(e) => Err(BusinessError::DecryptionError(format!("Error checking signature {}", e))),
    }
}

pub fn get_public_key(secret: &String, password: &String) -> String {
    let secret_vec = secret.as_bytes().to_vec();
    let seed: Vec<u8> = get_seed(&secret_vec, &password);
    let secret_key: SecretKey = SecretKey::from_slice(&seed[0..32]).expect("Got an error");
    let public_key: PublicKey = secret_key.public_key();
    base64.encode(public_key.to_encoded_point(true).as_bytes())
}

pub fn gen_base64_secret(hashed_otp: &String) -> String {
    let rand_bytes = gen_random_bytes();
    let seed = get_seed(&rand_bytes, &hashed_otp);
    let base64_seed = base64.encode(seed);
    base64_seed
}

#[cfg(test)]
mod tests {
    use super::{get_public_key, sign_msg, verify_msg};

    #[test]
    fn valid_sign() {
        let msg = "Message".to_string();
        let secret = "Secret".to_string();
        let password = "password".to_string();

        let sig = match sign_msg(&msg, &secret, &password) {
            Ok(sig) => sig,
            Err(e) => panic!("Expected a signature: {}", e),
        };

        let public_key = get_public_key(&secret, &password);

        match verify_msg(&msg, &sig, &public_key) {
            Ok(r) => assert!(r),
            Err(e) => panic!("Expected verify result: {}", e),
        }
    }
}