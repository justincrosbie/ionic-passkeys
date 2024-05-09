use std::sync::Arc;

use crate::db::credentials::{self, get_credentials_for_user};
use crate::db::regstates::Regstate;
use crate::db::users::{create_user, get_user_from_email, get_user_from_username_nocase};
use crate::db::{regstates, Db};

use shared::errors::{BusinessError, DbErrors};
use shared::json::auth::RegisterUser;
use shared::json::passkeys::{MyClientData, MyPasskeyAuthentication, MyPasskeyRegistration};
use webauthn_rs::prelude::{AuthenticationResult, Base64UrlSafeData, CreationChallengeResponse, Passkey, PasskeyAuthentication, PasskeyRegistration, PublicKeyCredential, RegisterPublicKeyCredential, RequestChallengeResponse, WebauthnError};
use webauthn_rs::Webauthn;

pub async fn start_registration_bl(db: &Db, username: String, displayname: String, webauthn: Arc<Webauthn>) ->  Result<CreationChallengeResponse, BusinessError> {

    let user = match get_user_from_email(&db, username.clone()).await {
        Ok(r) => r,
        Err(e) => {
            // call create_user to insert a new user into the database

            let new_register_user = RegisterUser {
                username: username.clone(),
                displayname: displayname.clone(),
                email: username.clone(),
                mobile: "".to_string(),
            };

            match create_user(&db, new_register_user, None).await {
                Ok(r) => r,
                Err(e) => {
                    return Err(BusinessError::DbError(DbErrors::User(e.to_string())))
                }
            }
        },
    };

    let user_credentials = match get_credentials_for_user(db, user.uuid).await {
        Ok(r) => r,
        Err(e) => {
            return Err(BusinessError::DbError(DbErrors::Credential(e.to_string())))
        }
    };

    let exclude_credentials = Some(user_credentials.iter().map(|c| c.get_credentialid()).collect());

    match webauthn.start_passkey_registration(
        user.uuid,
        &username,
        &displayname,
        exclude_credentials,
    ) {
        Ok((ccr, reg_state)) => {
            // Note that due to the session store in use being a server side memory store, this is
            // safe to store the reg_state into the session since it is not client controlled and
            // not open to replay attacks. If this was a cookie store, this would be UNSAFE.
            // session
            //     .insert("reg_state", (username, user_unique_id, reg_state))
            //     .expect("Failed to insert");
            // info!("Registration Successful!");

            let reg_state_str_result = serde_json::to_string(&reg_state);
            match reg_state_str_result {
                Ok(reg_state_str) => {
                    match persist_reg_state(reg_state_str, db, user, username).await {
                        Ok(r) => r,
                        Err(e) => {
                            return Err(BusinessError::DbError(DbErrors::Regstate(e.to_string())));
                        }
                    };
                },
                Err(e) => {
                    return Err(BusinessError::DbError(DbErrors::User(e.to_string())))
                }
            };

            Ok(ccr.into())
        }
        Err(e) => {
            debug!("challenge_register -> {:?}", e);
            return Err(BusinessError::WebauthnError(WebauthnError::from(e).to_string()));
        }
    }
}

async fn persist_reg_state(reg_state_str: String, db: &Db, user: crate::db::users::User, username: String) -> Result<Regstate, BusinessError> {

    println!("persist_state: {:?}", reg_state_str);

    let my_reg_state: MyPasskeyRegistration = match serde_json::from_str(&reg_state_str) {
        Ok(r) => r,
        Err(e) => {
            return Err(BusinessError::DbError(DbErrors::User(e.to_string())));
        },
    };
    let challenge_str = match serde_json::to_string(&my_reg_state.rs.challenge.clone()) {
        Ok(r) => r.replace("\"", ""),
        Err(e) => {
            return Err(BusinessError::DbError(DbErrors::User(e.to_string())));
        }
    };
    match regstates::create_regstate(&db, reg_state_str, challenge_str, user.uuid, username).await {
        Ok(r) => Ok(r),
        Err(e) => {
            return Err(BusinessError::DbError(DbErrors::User(e.to_string())));
        }
    }
}

async fn persist_auth_state(reg_state_str: String, db: &Db, user: crate::db::users::User, username: String) -> Result<Regstate, BusinessError> {

    println!("persist_state: {:?}", reg_state_str);

    let my_reg_state: MyPasskeyAuthentication = match serde_json::from_str(&reg_state_str) {
        Ok(r) => r,
        Err(e) => {
            return Err(BusinessError::DbError(DbErrors::User(e.to_string())));
        },
    };
    let challenge_str = match serde_json::to_string(&my_reg_state.ast.challenge.clone()) {
        Ok(r) => r.replace("\"", ""),
        Err(e) => {
            return Err(BusinessError::DbError(DbErrors::User(e.to_string())));
        }
    };
    match regstates::create_regstate(&db, reg_state_str, challenge_str, user.uuid, username).await {
        Ok(r) => Ok(r),
        Err(e) => {
            return Err(BusinessError::DbError(DbErrors::User(e.to_string())));
        }
    }
}

pub async fn complete_registration_bl(db: &Db, username: String, req: &RegisterPublicKeyCredential, webauthn: Arc<Webauthn>) -> Result<Passkey, BusinessError> {

    println!("complete_registration_bl: {:?}", req);
    let user = match get_user_from_username_nocase(&db, username.clone()).await {
        Ok(r) => r,
        Err(e) => {
            return Err(BusinessError::DbError(DbErrors::User(e.to_string())))
        },
    };

    println!("complete_registration_bl, got user: {:?}", user);

    let cdj = req.response.client_data_json.clone();

    // let cdj = base64::decode(cdj_str).unwrap();

    let cdj_str2 = String::from_utf8(cdj.into()).unwrap();

    println!("Client Data JSON: {:?}", cdj_str2);
    
    let my_client_data: MyClientData = match serde_json::from_str(&cdj_str2) {
        Ok(r) => r,
        Err(e) => {
            return Err(BusinessError::DbError(DbErrors::User(e.to_string())))
        }
    };

    let ch_str = my_client_data.challenge.clone().replace("\"", "");
    
    println!("Client Data challenge: {:?}", ch_str);

    let regstate = match regstates::get_regstate_by_challenge(&db, ch_str).await {
        Ok(r) => r,
        Err(e) => {
            println!("complete_registration_bl, get_regstate_by_challenge error: {:?}", e);
            return Err(BusinessError::DbError(DbErrors::Regstate(e.to_string())))
        },
    };

    println!("complete_registration_bl, got regstate: {:?}", regstate);
    
    // delete the regstate
    match regstates::delete_regstate(&db, regstate.uuid).await {
        Ok(r) => r,
        Err(e) => {
            return Err(BusinessError::DbError(DbErrors::Regstate(e.to_string())));
        }
    };

    let reg_state: PasskeyRegistration = match serde_json::from_str(&regstate.reg_state) {
        Ok(r) => r,
        Err(e) => {
            return Err(BusinessError::DbError(DbErrors::User(e.to_string())))
        },
    };

    println!("complete_registration_bl, decoded regstate: {:?}", reg_state);
    
    match webauthn.finish_passkey_registration (
        req,
        &reg_state,
    ) {
        Ok(sk) => {
            println!("Registration Successful!");

            let credentialid = match serde_json::to_string(&sk.cred_id()) {
                Ok(r) => r.replace("\"", ""),
                Err(e) => {
                    return Err(BusinessError::DbError(DbErrors::Credential(e.to_string())));
                }
            };
            let credential_str = match serde_json::to_string(&sk) {
                Ok(r) => r,
                Err(e) => {
                    return Err(BusinessError::DbError(DbErrors::Credential(e.to_string())));
                }
            };

            match credentials::create_credential(&db, user.uuid, credentialid, credential_str).await {
                Ok(r) => r,
                Err(e) => {
                    return Err(BusinessError::DbError(DbErrors::Credential(e.to_string())));
                }
            };

            println!("Credential OK: {:?}", sk);
            Ok(sk)
        }
        Err(e) => {
            println!("WebauthnError!! challenge_register -> {:?}", e);
            return Err(BusinessError::WebauthnError(WebauthnError::from(e).to_string()));
        }
    }
}

pub async fn start_authentication_bl(db: &Db, username: String, webauthn: Arc<Webauthn>) -> Result<RequestChallengeResponse, BusinessError> {

    println!("start_authentication_bl, username: {:?}", username);

    let user = match get_user_from_username_nocase(&db, username.clone()).await {
        Ok(r) => r,
        Err(e) => {
            return Err(BusinessError::DbError(DbErrors::User(e.to_string())))
        },
    };

    println!("start_authentication_bl, got user: {:?}", user);

    let user_credentials = match get_credentials_for_user(db, user.uuid).await {
        Ok(r) => r,
        Err(e) => {
            return Err(BusinessError::DbError(DbErrors::Credential(e.to_string())))
        },
    };

    println!("start_authentication_bl, got user_credentials: {:?}", user_credentials);

    let allow_credentials: Vec<webauthn_rs::prelude::Passkey> = user_credentials.iter().map(|c| c.get_passkey_as_passkey()).collect();

    let res = match webauthn.start_passkey_authentication(&allow_credentials)
    {
        Ok((rcr, auth_state)) => {
            // // Drop the mutex to allow the mut borrows below to proceed
            // drop(users_guard);

            // Note that due to the session store in use being a server side memory store, this is
            // safe to store the auth_state into the session since it is not client controlled and
            // not open to replay attacks. If this was a cookie store, this would be UNSAFE.
            // session
            //     .insert("auth_state", (user_unique_id, auth_state))
            //     .expect("Failed to insert");

            println!("Start Authentication Successful!");

            let reg_state_str_result = serde_json::to_string(&auth_state);
            match reg_state_str_result {
                Ok(reg_state_str) => {
                    match persist_auth_state(reg_state_str, db, user, username).await {
                        Ok(r) => r,
                        Err(e) => {
                            return Err(BusinessError::DbError(DbErrors::Regstate(e.to_string())));
                        }
                    };
                },
                Err(e) => {
                    return Err(BusinessError::DbError(DbErrors::Regstate(e.to_string())))
                }
            };

            rcr
        }
        Err(e) => {
            debug!("challenge_authenticate -> {:?}", e);
            return Err(BusinessError::WebauthnError(WebauthnError::from(e).to_string()));
        }
    };
    Ok(res)    
}

pub async fn complete_authentication_bl(db: &Db, auth: &PublicKeyCredential, webauthn: Arc<Webauthn>) -> Result<AuthenticationResult, BusinessError> {

    println!("complete_authentication_bl: {:?}", auth);
    

    let cdj = auth.response.client_data_json.clone();
    let cdj_str2 = String::from_utf8(cdj.into()).unwrap();

    println!("Client Data JSON: {:?}", cdj_str2);
    
    let my_client_data: MyClientData = match serde_json::from_str(&cdj_str2) {
        Ok(r) => r,
        Err(e) => {
            return Err(BusinessError::DbError(DbErrors::User(e.to_string())))
        }
    };

    println!("Client Data origin: {:?}", my_client_data.origin.clone());
    println!("Client Data challenge: {:?}", my_client_data.challenge.clone());

    let regstate = match regstates::get_regstate_by_challenge(&db, my_client_data.challenge.clone()).await {
        Ok(r) => r,
        Err(e) => {
            println!("complete_authentication_bl, get_regstate_by_challenge error: {:?}", e);
            return Err(BusinessError::DbError(DbErrors::Regstate(e.to_string())))
        },
    };

    println!("complete_authentication_bl, got regstate: {:?}", regstate);
    
    // delete the regstate
    match regstates::delete_regstate(&db, regstate.uuid).await {
        Ok(r) => r,
        Err(e) => {
            return Err(BusinessError::DbError(DbErrors::Regstate(e.to_string())));
        }
    };

    let auth_state: PasskeyAuthentication = match serde_json::from_str(&regstate.reg_state) {
        Ok(r) => r,
        Err(e) => {
            return Err(BusinessError::DbError(DbErrors::User(e.to_string())))
        },
    };

    println!("complete_authentication_bl, decoded regstate: {:?}", auth_state);
    
    match webauthn.finish_passkey_authentication (
        &auth,
        &auth_state,
    ) {
        Ok(ar) => {
            println!("Authentication Successful!");

            let user_credentials = match get_credentials_for_user(db, regstate.user_uuid).await {
                Ok(r) => r,
                Err(e) => {
                    return Err(BusinessError::DbError(DbErrors::Credential(e.to_string())))
                },
            };
        
            let _ = user_credentials.iter().map(|c| c.get_passkey_as_passkey().update_credential(&ar));
        
            Ok(ar)
        }
        Err(e) => {
            println!("WebauthnError!! challenge_authenticate -> {:?}", e);
            return Err(BusinessError::WebauthnError(WebauthnError::from(e).to_string()));
        }
    }
}
