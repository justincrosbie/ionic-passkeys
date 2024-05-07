use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use crate::schema::regstates;

use super::Db;

#[derive(Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = regstates)]
/*
create table REGSTATES (
    uuid UUID PRIMARY KEY,
    user_uuid UUID NOT NULL,
    username VARCHAR NOT NULL,
    reg_state TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    modified_at TIMESTAMP
);
 */
pub struct Regstate {
    pub uuid: uuid::Uuid,
    pub user_uuid: uuid::Uuid,
    pub username: String,
    pub challenge: String,
    pub reg_state: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: Option<chrono::NaiveDateTime>,
}

impl Regstate {

    pub fn from_reg_state_and_user_uuid(reg_state: String, challenge: String, user_uuid: uuid::Uuid, username: String) -> Self {
        Regstate {
            uuid: uuid::Uuid::new_v4(),
            user_uuid: user_uuid,
            username: username,
            challenge: challenge,
            reg_state: reg_state,
            created_at: chrono::offset::Local::now().naive_local(),
            modified_at: None,
        }
    }
    // pub fn from_json_credential(json_credential: StartRegistrationRequest, user_uuid: uuid::Uuid) -> Self {
    //     Credential {
    //         uuid: uuid::Uuid::new_v4(),
    //         user_uuid: user_uuid,
    //         credentialid: Some(json_credential.credentialid),
    //         passkey: None,
    //         reg_state: "STARTED".to_string(),
    //         created_at: chrono::offset::Local::now().naive_local(),
    //         modified_at: chrono::offset::Local::now().naive_local(),
    //     }
    // }

    // pub fn to_json_credential(&self) -> StartRegistrationResponse {
    //     StartRegistrationResponse {
    //         uuid: self.uuid,
    //         credentialid: self.credentialid.clone().unwrap(),
    //         reg_state: self.reg_state.clone(),
    //         created_at: self.created_at.timestamp_millis(),
    //         modified_at: self.modified_at.timestamp_millis(),
    //     }
    // }
}

pub async fn create_regstate(db: &Db, req_state_str: String, challenge: String, user_uuid: uuid::Uuid, username: String) -> Result<Regstate, Error> {
    let regstate = Regstate::from_reg_state_and_user_uuid(req_state_str, challenge, user_uuid, username);
    db.run(move |conn| {
        diesel::insert_into(regstates::table)
        .values(&regstate)
        .get_result(conn)
    }).await
}

pub async fn get_regstate_by_username(db: &Db, username: String) -> Result<Regstate, Error> {
    db.run(move |conn| {
        regstates::table.filter(regstates::username.eq(username))
        .get_result(conn)
    }).await
}

pub async fn get_regstate_by_challenge(db: &Db, challenge: String) -> Result<Regstate, Error> {
    db.run(move |conn| {
        regstates::table.filter(regstates::challenge.eq(challenge))
        .get_result(conn)
    }).await
}

pub async fn delete_regstate_by_username(db: &Db, username: String) -> Result<usize, Error> {
    db.run(move |conn| {
        diesel::delete(regstates::table.filter(regstates::username.eq(username)))
        .execute(conn)
    }).await
}

pub async fn delete_regstate(db: &Db, regstate_uuid: uuid::Uuid) -> Result<usize, Error> {
    db.run(move |conn| {
        diesel::delete(regstates::table.filter(regstates::uuid.eq(regstate_uuid)))
        .execute(conn)
    }).await
}