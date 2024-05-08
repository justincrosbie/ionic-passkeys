use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;

use uuid;
use crate::schema::users;

use super::Db;
use shared::json::user::{UserState, User as Usr};
use shared::json::auth::RegisterUser as RegUsr;

#[derive(Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub uuid: uuid::Uuid,
    pub username: String,
    pub displayname: String,
    pub email: String,
    pub mobile: String,
    pub public_key: String,
    pub user_state: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: Option<chrono::NaiveDateTime>,
    pub verified: bool,
}

impl User {
    fn from_json_reg_user(json_user: RegUsr, uuid: Option<uuid::Uuid>) -> Self {
        User {
            uuid: match uuid { 
                Some(u) => u, 
                None => uuid::Uuid::new_v4(),
            },
            username: json_user.username,
            displayname: json_user.displayname,
            email: json_user.email,
            mobile: json_user.mobile,
            public_key: "UNKNOWN".to_string(),
            user_state: "PREPROVISIONED".to_string(),
            created_at: chrono::offset::Local::now().naive_local(),
            modified_at: Some(chrono::offset::Local::now().naive_local()),
            verified: false,
        }        
    }

    fn to_json_user(&self) -> Usr {
        let user_state = match self.user_state.as_str() {
            "USER" => UserState::USER,
            "ADMIN" => UserState::ADMIN,
            "ARBITRATOR" => UserState::ARBITRATOR,
            "PREPROVISIONED" => UserState::PREPROVISIONED,
            _ => UserState::USER,
        };
        Usr {
            uuid: self.uuid,
            username: self.username.to_string(),
            displayname: self.displayname.to_string(),
            email: self.email.to_string(),
            mobile: self.mobile.to_string(),
            user_state: user_state,
            created_at: self.created_at,
            modified_at: self.modified_at,
            verified: self.verified,
        }
    }
}

pub async fn create_user(db: &Db, reg_user: RegUsr, uuid: Option<uuid::Uuid>) -> Result<User, Error> {
    let new_user = User::from_json_reg_user(reg_user, uuid);
    let user: Result<User, Error> = db.run(move |conn| {
        diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
    }).await;

    user
}

/*
pub async fn get_user_from_username_with_secret(db: &Db, passed_username: String) -> Result<User, Error> {
    use self::users::dsl::*;

    let user: Result<User, Error> = db.run(move | conn | {
        users
        .filter(username.eq(&passed_username))
        .get_result(conn)
    }).await;

    user
}
*/

pub async fn get_user_by_username_email_or_mobile(db: &Db, passed_user: String) -> Result<User, Error> {
    use self::users::dsl::*;

    // Removed as the mobile can be empty and the first empty mobile could see that user spammed.
    // .or(mobile.eq(&passed_user))
    let user: Result<User,Error> = db.run(move |conn | {
        users
        .filter(username.eq(&passed_user).or(email.eq(&passed_user)))
        .get_result(conn)
    }).await;

    user
}
pub async fn get_user_from_email(db: &Db, passed_email: String) -> Result<User, Error> {
    use self::users::dsl::*;

    let user: Result<User, Error> = db.run(move | conn | {
        users
        .filter(email.eq(&passed_email))
        .get_result(conn)
    }).await;

    user
}

pub async fn get_user_from_username_nocase(db: &Db, passed_username: String) -> Result<User, Error> {

    use self::users::dsl::*;

    let user: Result<User, Error> = db.run(move | conn | {
        users
        // filter the username with a case insensitive match
        .filter(username.ilike(&passed_username))
        .get_result(conn)
    }).await;

    user
}

pub async fn get_db_user_from_uuid(db: &Db, passed_uuid: uuid::Uuid) -> Result<User, Error> {
    use self::users::dsl::*;

    let user: Result<User, Error> = db.run(move | conn | {
        users
        .filter(uuid.eq(&passed_uuid))
        .get_result(conn)
    }).await;

    user
}

pub async fn modify_state_key(db: &Db, passed_uuid: uuid::Uuid, passed_user_state: UserState, passed_public_key: String) -> Result<User,Error> {
    let state = match passed_user_state {
        UserState::USER => "USER",
        UserState::ADMIN => "ADMIN",
        UserState::ARBITRATOR => "ARBITRATOR",
        UserState::PREPROVISIONED => "PREPROVISIONED",
        _ => "USER",
    };

    use self::users::dsl::*;

    let user: Result<User,Error> = db.run(move |conn| {
        diesel::update(users)
        .filter(uuid.eq(&passed_uuid))
        .set((user_state.eq(state),
            public_key.eq(passed_public_key))
        )
        .get_result(conn)
    }).await;

    user
}

pub async fn get_users_by_status(db: &Db, passed_user_state: UserState) -> Result<Vec<User>,Error> {
    let state = match passed_user_state {
        UserState::USER => "USER",
        UserState::ADMIN => "ADMIN",
        UserState::ARBITRATOR => "ARBITRATOR",
        UserState::PREPROVISIONED => "PREPROVISIONED",
        _ => "USER",
    };

    use self::users::dsl::*;

    let user: Result<Vec<User>,Error> = db.run(move | conn | {
        users
        .filter(user_state.eq(&state))
        .get_results(conn)
    }).await;

    user

}

// pub async fn update_user_info(db: &Db, user_uuid: uuid::Uuid, passed_user_update: UpdateUser) -> Result<User, Error> {
//     use self::users::dsl::*;

//     let od = if  passed_user_update.otp_delivery.eq(&OtpDelivery::SMS) { "SMS"} else { "EMAIL"};
//     let user: Result<User, Error> = db.run(move |conn| {
//         diesel::update(users)
//         .filter(uuid.eq(&user_uuid))
//         .set((email.eq(&passed_user_update.email),
//             mobile.eq(&passed_user_update.mobile),
//             otp_delivery.eq(od),
//             pfp_url.eq(&passed_user_update.pfp_url),
//         )
//         )
//         .get_result(conn)
//     }).await;

//     user
// }

// pub async fn modify_user_info(db: &Db, passed_user_modify: Usr) -> Result<User, Error> {
//     use self::users::dsl::*;

//     let od = if  passed_user_modify.otp_delivery.eq(&OtpDelivery::SMS) { "SMS"} else { "EMAIL"};
//     let state = match passed_user_modify.user_state {
//         UserState::USER => "USER",
//         UserState::ADMIN => "ADMIN",
//         UserState::ARBITRATOR => "ARBITRATOR",
//         UserState::PREPROVISIONED => "PREPROVISIONED",
//         _ => "USER",
//     };

//     let user: Result<User, Error> = db.run(move |conn| {
//         diesel::update(users)
//         .filter(uuid.eq(&passed_user_modify.uuid))
//         .set((email.eq(&passed_user_modify.email),
//             mobile.eq(&passed_user_modify.mobile),
//             otp_delivery.eq(od),
//             user_state.eq(state)
//         )
//         )
//         .get_result(conn)
//     }).await;

//     user
// }

pub async fn get_users(db: &Db, name_starts: String) -> Result<Vec<Usr>, Error> {
    use self::users::dsl::*;

    let user: Result<Vec<User>, Error> = db.run(move | conn | {
        users
        .filter(username.like(&(name_starts + "%")))
        .get_results(conn)
    }).await;

    user.map(|value| value.iter().map(|user| user.to_json_user()).collect())
}

// pub async fn modify_user_secret(db: &Db, user_uuid: uuid::Uuid, passed_modified_secret: ModifiedSecret) -> Result<User,Error> {
//     use self::users::dsl::*;

//     let user: Result<User, Error> = db.run(move |conn| {
//         diesel::update(users)
//         .filter(uuid.eq(&user_uuid))
//         .set((secret.eq(&passed_modified_secret.secret),
//             public_key.eq(&passed_modified_secret.public_key),
//         )
//         )
//         .get_result(conn)
//     }).await;

//     user

// }