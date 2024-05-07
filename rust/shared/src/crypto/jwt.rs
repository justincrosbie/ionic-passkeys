use serde::{Deserialize, Serialize};
use chrono::Utc;
use base64::{Engine as _, engine::general_purpose::URL_SAFE as base64};

use crate::{json::user::{UserState, User}, errors::BusinessError};

use super::{auth::{sign_msg, verify_msg}, helpers::hash160};

///
/// JWT Header
/// 
/// This is a basic formatted JWT header with just the two required attributes
/// 
#[derive(Deserialize, Serialize)]
struct JwtHeader {
    typ: String,
    alg: String,
}
///
/// JWT Payload body
/// 
/// This is the payload (claims section) of the JWT with our specific attributes
/// 
#[derive(Deserialize, Serialize)]
pub struct JwtPayload {
    //sub: String, // Principal of the claim (username)
    iss: uuid::Uuid, // UUID of the user
    role: UserState, // Role he user has
    jti: String, // Random nonce to stop replay attacks
    exp: i64, // EPOCH in sec till expiry
    iat: i64, // EPOCH of issued at time   
}

///
/// The entire JWT structure
/// 
/// This is here to help us translate between our JWT string and JWT claims
/// 
#[derive(Deserialize, Serialize)]
struct Jwt {
    header: JwtHeader,
    payload: JwtPayload,
    signatue: String,
}

///
/// Implementation methods for the JWT payload
/// 
/// These are used to convert our Payload to a JWT and back again
/// 
impl JwtPayload {

    pub fn create(/*sub: String,*/ iss: uuid::Uuid, role: UserState) -> Result<Self, BusinessError> {

        let now = Utc::now().timestamp();

        Ok(JwtPayload { 
            //sub: sub, 
            iss: iss, 
            role: role, 
            jti: gen_jti(iss, now), 
            exp: 10 + now,
            iat:  now,
        })
    }

    /// This signs the JwtPayload and returns a JWT token.
    /// 
    /// The token created is signed using EC256K, it is the only format that is supported right now.
    /// A JWT base64 encoded dot format is returned in the form of "<header>.<payload>.<signature>"
    /// 
    pub fn sign(&self, secret: &String, password: &String) -> Result<String, BusinessError> {
        
        // We hardcode EC256K because that is what we want to use
        let header: String = match serde_json::to_string(&JwtHeader {
            typ: "JWT".to_string(),
            alg: "EC256K".to_string(),
        }) {
            Ok(header) => header,
            Err(e) => return Err(BusinessError::DecryptionError(format!("Unable to flatten JWT header: {}", e))),
        };

        // Flatten the payload
        let payload: String = match serde_json::to_string(self) {
            Ok(payload) => payload,
            Err(e) => return Err(BusinessError::DecryptionError(format!("Unable to flatten JWT payload: {}", e))),
        };

        // Encode the message for signing
        let message: String = format!("{}.{}", 
            base64.encode(header.as_bytes()),
            base64.encode(payload.as_bytes()) 
        );

        // Sign the header + payload
        let base64_signature: String = match sign_msg(&message, &secret, &password) {
            Ok(sig) => sig,
            Err(e) => return Err(e),
        };

        // Return our token
        Ok(format!("{}.{}", message, base64_signature))
    }

    /// Verify the passed token
    /// 
    /// We only support a alg of EC256K for signatures, as this is currently a closed system.
    /// The JWT Payload (Claims) struct are returned after basic verifcation is performed.
    /// If it fails verification eg: Expired or signature doesn't match etc then an error is returned.
    /// 
    pub fn verify(token: &String, public_key_base64: &String, user: &User) -> Result<Self, BusinessError> {

        // Verify token has correct number of "."
        let output: Vec<&str> = token.split(".").collect();
        if output.len() != 3 {
            return Err(BusinessError::JwtError("Token is in the incorrect format".to_string()))
        }
        let base64_header:    String = output[0].to_string();
        let base64_payload:   String = output[1].to_string();
        let base64_signature: String = output[2].to_string();

        // Verify the signature
        if !match verify_msg(&format!("{}.{}", base64_header, base64_payload), &base64_signature, &public_key_base64) {
            Ok(b) => b,
            Err(e) => return Err(e),
        } {
            return Err(BusinessError::JwtError("Signature does not match message".to_string()))
        }

        // Verify the header (the lazy way)
        let header: Vec<u8> = match base64.decode(base64_header) {
            Ok(header) => header,
            Err(e) => return Err(BusinessError::JwtError(format!("Header not in base64 format : {}", e))),
        };
        let jwt_header: JwtHeader = match serde_json::from_slice::<JwtHeader>(&header) {
            Ok(header) => header,
            Err(e) => return Err(BusinessError::JwtError(format!("Not a JwtHeader: {}", e))),
        };
        if !"EC256K".eq(&jwt_header.alg) || !"JWT".eq(&jwt_header.typ){
            return Err(BusinessError::JwtError("JWT Header is not what was epected".to_string()))
        }

        let payload: Vec<u8> = match base64.decode(base64_payload) {
            Ok(payload) => payload,
            Err(e) => return Err(BusinessError::JwtError(format!("Payload not in base64 format : {}", e))),
        };

        // TODO: Verify the expiry and issue ita and jti
        let jwt_payload = match serde_json::from_slice::<JwtPayload>(&payload) {
            Ok(jwt_payload) => jwt_payload,
            Err(e) => return Err(BusinessError::JwtError(format!("Not a JwtPayload: {}", e))),
        };
        if jwt_payload.iss.ne(&user.uuid)
            //|| jwt_payload.sub.ne(&user.username)
            || jwt_payload.role.ne(&user.user_state) 
        {
            Err(BusinessError::JwtError("Jwt doesn't match up with our records".to_string()))
        } else {
            Ok(jwt_payload)
        }
    }

    ///
    /// Get the issuer uuid
    /// 
    /// This pulls apart the JWT and gets the issuer without checking that the values are valid,
    /// except for the token being in the three parts, and the payload is in the correct format.
    /// 
    pub fn get_iss(token: &str) -> Result<uuid::Uuid, BusinessError> {
        let output: Vec<&str> = token.split(".").collect();
        if output.len() != 3 {
            return Err(BusinessError::JwtError("Token is in the incorrect format".to_string()))
        }
        let base64_payload:   String = output[1].to_string();

        let payload: Vec<u8> = match base64.decode(base64_payload) {
            Ok(payload) => payload,
            Err(e) => return Err(BusinessError::JwtError(format!("Payload not in base64 format : {}", e))),
        };

        let jwt_payload = match serde_json::from_slice::<JwtPayload>(&payload) {
            Ok(jwt_payload) => jwt_payload,
            Err(e) => return Err(BusinessError::JwtError(format!("Not a JwtPayload: {}", e))),
        };
        Ok(jwt_payload.iss)
    }
}

fn gen_jti(iss: uuid::Uuid, time: i64) -> String {
    base64.encode(hash160(&format!("{}-{}",iss, time).as_bytes().to_vec()))
}