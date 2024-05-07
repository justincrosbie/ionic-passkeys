use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use webauthn_rs::prelude::CredentialID;

use crate::schema::credentials;

use super::Db;

#[derive(Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = credentials)]
pub struct Credential {
    pub uuid: uuid::Uuid,
    pub user_uuid: uuid::Uuid,
    pub credentialid: String,
    pub passkey: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: Option<chrono::NaiveDateTime>,
}

impl Credential {

    pub fn from_args(uuid: uuid::Uuid, user_uuid: uuid::Uuid, credentialid: String, passkey: String) -> Self {
        Credential {
            uuid: uuid,
            user_uuid: user_uuid,
            credentialid: credentialid,
            passkey: passkey,
            created_at: chrono::offset::Local::now().naive_local(),
            modified_at: None,
        }
    }

    pub fn get_credentialid(&self) -> CredentialID {

        println!("CredentialID: {}", self.credentialid);

        // surround self.credentialid with doule-quotes if not already
        let mut cred = self.credentialid.clone();
        if !cred.starts_with("\"") {
            cred = format!("\"{}\"", cred);
        }

        match serde_json::from_str(&cred) {
            Ok(c) => c,
            Err(e) => {
                panic!("Unable to parse credentialid: {}", e);
            }
        }
    }

    pub fn get_passkey_as_passkey(&self) -> webauthn_rs::prelude::Passkey {
        println!("Passkey str: {:?}", &self.passkey);

        let passkey: webauthn_rs::prelude::Passkey = serde_json::from_str(&self.passkey).unwrap();

        println!("Passkey: {:?}", passkey);
        passkey
    }
}

pub async fn create_credential(db: &Db, user_uuid: uuid::Uuid, credentialid: String, passkey: String) -> Result<Credential, Error> {
    let uuid = uuid::Uuid::new_v4();
    let credential = Credential::from_args(uuid, user_uuid, credentialid, passkey);
    db.run(move |conn| {
        diesel::insert_into(credentials::table)
        .values(&credential)
        .get_result(conn)
    }).await
}

pub async fn get_credential_from_uuid(db: &Db, credential_uuid: uuid::Uuid) -> Result<Credential, Error> {
    db.run(move |conn| {
        credentials::table
        .filter(credentials::uuid.eq(credential_uuid))
        .get_result(conn)
    }).await
}

pub async fn get_credentials_for_user(db: &Db, user_uuid: uuid::Uuid) -> Result<Vec<Credential>, Error> {
    db.run(move |conn| {
        credentials::table
        .filter(credentials::user_uuid.eq(user_uuid))
        .load::<Credential>(conn)
    }).await
}

pub async fn delete_credential(db: &Db, credential_uuid: uuid::Uuid) -> Result<usize, Error> {
    db.run(move |conn| {
        diesel::delete(credentials::table.filter(credentials::uuid.eq(credential_uuid)))
        .execute(conn)
    }).await
}