use rocket::{response::status, serde::json::Json};
use rocket::State;
use shared::json::passkeys::{CompleteRegistrationRequest, CompleteRegistrationResponse, StartAuthenticationRequest, StartAuthenticationResponse, StartRegistrationRequest, StartRegistrationResponse, CompleteAuthenticationRequest, CompleteAuthenticationResponse};
use crate::engine::passkeys::{complete_authentication_bl, complete_registration_bl, start_authentication_bl, start_registration_bl};
use crate::engine::startup::AppState;
use crate::db::Db;

#[get("/hello")]
pub fn hello() -> &'static str {
    "Chaka Kahn, let me rock you!"
}

#[post("/start_registration", format = "json", data = "<request>")]
pub async fn start_registration(db: Db, state: &State<AppState>, request: Json<StartRegistrationRequest>) -> Result<Json<StartRegistrationResponse>, status::BadRequest<String>> {

    let email = request.user_email.clone();
    let webauthn = state.webauthn.clone(); // Clone the state.webauthn value

    let result = start_registration_bl(&db, email, webauthn).await;
    match result {
        Ok(r) => Ok(Json(StartRegistrationResponse::new(r))),
        Err(e) => return Err(status::BadRequest(e.to_string())),
    }
}

#[post("/complete_registration", format = "json", data = "<request>")]
pub async fn complete_registration(db: Db, state: &State<AppState>, request: Json<CompleteRegistrationRequest>) -> Result<Json<CompleteRegistrationResponse>, status::BadRequest<String>> {

    println!("Complete Registration Request: {:?}", request);

    let webauthn = state.webauthn.clone(); // Clone the state.webauthn value

    // let credential = serde_json::from_str::<RegisterPublicKeyCredential>(&request.credential);
    // let credential = match credential {
    //     Ok(c) => c,
    //     Err(e) => return Err(status::BadRequest(e.to_string())),
    // };

    let credential = request.credential.clone();
    let result = complete_registration_bl(&db, request.username.clone(), &credential, webauthn).await;
    match result {
        Ok(r) => Ok(Json(CompleteRegistrationResponse { success: true, message: "Registration successful.".to_string() })),
        Err(e) => return Err(status::BadRequest(e.to_string())),
    }
}

#[post("/start_authentication", format = "json", data = "<request>")]
pub async fn start_authentication(db: Db, state: &State<AppState>, request: Json<StartAuthenticationRequest>) -> Result<Json<StartAuthenticationResponse>, status::BadRequest<String>> {

    let username = request.username.clone();
    let webauthn = state.webauthn.clone(); // Clone the state.webauthn value

    let result = start_authentication_bl(&db, username, webauthn).await;
    match result {
        Ok(r) => Ok(Json(StartAuthenticationResponse::new(r))),
        Err(e) => return Err(status::BadRequest(e.to_string())),
    }
}

#[post("/complete_authentication", format = "json", data = "<request>")]
pub async fn complete_authentication(db: Db, state: &State<AppState>, request: Json<CompleteAuthenticationRequest>) -> Result<Json<CompleteAuthenticationResponse>, status::BadRequest<String>> {

    println!("Complete Registration Request: {:?}", request);

    let webauthn = state.webauthn.clone(); // Clone the state.webauthn value

    // let credential = serde_json::from_str::<RegisterPublicKeyCredential>(&request.credential);
    // let credential = match credential {
    //     Ok(c) => c,
    //     Err(e) => return Err(status::BadRequest(e.to_string())),
    // };

    let credential = request.credential.clone();
    let result = complete_authentication_bl(&db, &credential, webauthn).await;
    match result {
        Ok(r) => Ok(Json(CompleteAuthenticationResponse { success: true, message: "Registration successful.".to_string() })),
        Err(e) => return Err(status::BadRequest(e.to_string())),
    }
}
