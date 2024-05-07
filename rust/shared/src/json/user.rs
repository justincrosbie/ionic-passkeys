use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use webauthn_rs::prelude::{AuthenticationResult, CredentialID, Passkey};

use crate::crypto::crypto;

/// HardwareSecurityKey abstracts hardware security key such as Yubikey.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareSecurityKey {
    pub id: String,
    // The name of key.
    pub name: String,
    // The kind of key.
    pub kind: String,
    // The secret for hardware key.
    pub key: Passkey,
    // recovery code
    pub recovery_code: String,
}

impl HardwareSecurityKey {
    pub fn new(name: &str, key: &Passkey) -> Self {
        Self {
            id: key.cred_id().to_string(),
            name: name.to_string(),
            kind: "webauthn".into(),
            key: key.clone(),
            recovery_code: crypto::generate_recovery_code(12),
        }
    }

    // recovery code should be one-time use only
    pub fn update_recovery(&mut self) {
        self.recovery_code = crypto::generate_recovery_code(12);
    }
}


#[derive(Debug, Deserialize,Serialize, Clone)]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub email: String,
    pub mobile: String,
    pub user_state: UserState,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: Option<chrono::NaiveDateTime>,
    pub verified: bool,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct UpdateUser {
    pub email: String,
    pub mobile: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct ModifiedSecret {
    pub secret: String,
    pub public_key: String,
}

#[wasm_bindgen]
#[derive(Deserialize,Serialize, Debug, Clone, PartialEq)]
pub enum UserState {
    PREPROVISIONED = "PREPROVISIONED",
    USER = "USER",
    ARBITRATOR = "ARBITRATOR",
    ADMIN = "ADMIN"
}

impl UserState {
/*
    pub fn from_str(name: &str) -> Self {
        match name {
            "PREPROVISIONED" => UserState::PREPROVISIONED,
            "USER" => UserState::USER,
            "ARBITRATOR" => UserState::ARBITRATOR,
            "ADMIN" => UserState::ADMIN,
            _ => UserState::USER,
        }
    }
 
    pub fn to_str(&self) -> String {
        match self {
            UserState::PREPROVISIONED => "PREPROVISIONED".to_string(),
            UserState::USER => "USER".to_string(),
            UserState::ARBITRATOR => "ARBITRATOR".to_string(),
            UserState::ADMIN => "ADMIN".to_string(),
        }
    }
*/
}
