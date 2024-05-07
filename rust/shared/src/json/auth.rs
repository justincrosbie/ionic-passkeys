use serde::{Serialize, Deserialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use super::user::{UserState};

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize,Serialize, Clone, Debug)]
pub struct LoginUser {
    pub username: String,
}

#[wasm_bindgen]
impl LoginUser {
    #[wasm_bindgen(constructor)]
    pub fn new(username: String) -> LoginUser {
        LoginUser { username: username }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize,Serialize, Clone, Debug)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub mobile: String
}

#[wasm_bindgen]
impl RegisterUser {
    #[wasm_bindgen(constructor)]
    pub fn new(username: String, email: String, mobile: String) -> RegisterUser {
        RegisterUser { 
            username: username,
            email: email,
            mobile: mobile
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OtpUser {
    pub user_uuid: Uuid,
    pub public_key: String,
    pub signature: String,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize,Serialize, Debug)]
pub struct OtpResponse {
    pub otp_payload: String,
    pub user_uuid: String,
    pub user_state: UserState,
    pub expire_at: i64,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Deserialize,Serialize, Debug)]
pub struct OtpBetaResponse {
    pub otp: String,
    pub otp_payload: String,
    pub user_uuid: String,
    pub user_state: UserState,
    pub expire_at: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BetaUser {
    pub username: String,
    pub email: String,
    pub country: String,
    pub company: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RegisterUserBeta {
    pub username: String,
    pub beta_key: uuid::Uuid,
}