use serde::{Serialize, Deserialize};

pub mod user;
pub mod passkeys;
pub mod auth;

#[derive(Deserialize,Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}