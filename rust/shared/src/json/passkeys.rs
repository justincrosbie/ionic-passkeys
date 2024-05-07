// use serde::{Serialize, Deserialize};
// use uuid::Uuid;
// use wasm_bindgen::prelude::*;

// use super::user::User;

// #[derive(Deserialize,Serialize, Clone, PartialEq)]
// pub struct Rp {
//     pub name: String,
//     pub id: String,
// }

// #[derive(Deserialize,Serialize, Clone, PartialEq)]
// pub struct RegistrationChallengeResponse {
//     pub challenge: String,
//     pub user: User,
//     pub rp: Rp,
//     pub timeout: i64,
// }

use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::{Base64UrlSafeData, CreationChallengeResponse, PublicKeyCredential, RegisterPublicKeyCredential, RequestChallengeResponse};

// #[derive(Serialize, Deserialize)]
// struct ChallengeRequest {
//     user_id: String,
// }

// #[derive(Serialize, Deserialize)]
// struct ChallengeResponse {
//     challenge: String,
//     // Include other necessary fields according to your application's needs
// }

// #[derive(Serialize, Deserialize)]
// struct VerificationRequest {
//     user_id: String,
//     public_key: String,
//     signature: String,
//     challenge: String,
//     // Include other fields as required by the WebAuthn protocol
// }

// #[derive(Serialize, Deserialize)]
// struct VerificationResponse {
//     verified: bool,
//     // Include other response fields as necessary
// }


// Request to start the registration process
#[derive(Deserialize,Serialize, Clone, Debug)]
pub struct StartRegistrationRequest {
    pub user_email: String,
}

// Response with registration challenge
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct StartRegistrationResponse {
    pub challenge: CreationChallengeResponse,
    // Include other necessary WebAuthn parameters
}

impl StartRegistrationResponse {
    pub fn new(challenge: CreationChallengeResponse) -> Self {
        Self {
            challenge,
        }
    }
}

// Request to complete the registration
#[derive(Deserialize,Serialize, Clone, Debug)]
pub struct CompleteRegistrationRequest {
    pub username: String,
    pub credential: RegisterPublicKeyCredential, // Simplified; in practice, this would include the client's response data
    // Include other fields as returned by the WebAuthn client
}

// Response to indicate registration completion status
#[derive(Deserialize,Serialize, Clone, Debug)]
pub struct CompleteRegistrationResponse {
    pub success: bool,
    pub message: String,
}

// Request to start the registration process
#[derive(Deserialize,Serialize, Clone, Debug)]
pub struct StartAuthenticationRequest {
    pub username: String,
}


// Response with registration challenge
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct StartAuthenticationResponse {
    pub challenge: RequestChallengeResponse,
    // Include other necessary WebAuthn parameters
}

impl StartAuthenticationResponse {
    pub fn new(challenge: RequestChallengeResponse) -> Self {
        Self {
            challenge,
        }
    }
}

// Request to complete the Authentication
#[derive(Deserialize,Serialize, Clone, Debug)]
pub struct CompleteAuthenticationRequest {
    pub credential: PublicKeyCredential, // Simplified; in practice, this would include the client's response data
    // Include other fields as returned by the WebAuthn client
}

// Response to indicate Authentication completion status
#[derive(Deserialize,Serialize, Clone, Debug)]
pub struct CompleteAuthenticationResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize,Serialize, Clone, Debug)]
pub struct MyPasskeyState {
    pub challenge: Base64UrlSafeData,
}
#[derive(Deserialize,Serialize, Clone, Debug)]
pub struct MyPasskeyRegistration {
    pub rs: MyPasskeyState,
}
#[derive(Deserialize,Serialize, Clone, Debug)]
pub struct MyPasskeyAuthentication {
    pub ast: MyPasskeyState,
}

#[derive(Deserialize,Serialize, Clone, Debug)]
pub struct MyClientData {
    pub challenge: String,
    pub origin: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "crossOrigin")]
    pub cross_origin: bool
}
