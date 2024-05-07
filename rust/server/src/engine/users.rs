use shared::json::user::{UpdateUser, User, ModifiedSecret}; 
use shared::json::GenericResponse;
use shared::errors::{BusinessError, DbErrors};

use crate::db::Db;
// use crate::db::users::{update_user_info, get_users, modify_user_info, modify_user_secret, get_db_user_from_uuid};
use crate::db::users::{get_users, get_db_user_from_uuid};

// pub async fn update_user(user_uuid: uuid::Uuid, db: &Db, updated_user: UpdateUser) -> Result<GenericResponse, BusinessError> {
//     match update_user_info(db, user_uuid, updated_user).await {
//         Ok(_) =>     Ok(GenericResponse {status: "SUCCESS".to_string(), message: "Updated".to_string()}),
//         Err(e) => Err(BusinessError::DbError(shared::errors::DbErrors::User(e.to_string())))
//     }
// }

// pub async fn modify_user(db: &Db, modified_user: User) -> Result<GenericResponse, BusinessError> {
//     match modify_user_info(db, modified_user).await {
//         Ok(_) =>     Ok(GenericResponse {status: "SUCCESS".to_string(), message: "Updated".to_string()}),
//         Err(e) => Err(BusinessError::DbError(shared::errors::DbErrors::User(e.to_string())))
//     }
// }

pub async fn get_users_startwith(db: &Db, starts_with: String) -> Result<Vec<User>, BusinessError> {
    match get_users(db, starts_with).await {
        Ok(users) => Ok(users),
        Err(e) => Err(BusinessError::DbError(shared::errors::DbErrors::User(e.to_string()))),
    }
}

// pub async fn modify_secret(user_uuid: uuid::Uuid, db: &Db, modified_secret: ModifiedSecret) -> Result<GenericResponse, BusinessError> {
//     match modify_user_secret(db, user_uuid, modified_secret).await {
//         Ok(_) =>     Ok(GenericResponse {status: "SUCCESS".to_string(), message: "Updated".to_string()}),
//         Err(e) => Err(BusinessError::DbError(shared::errors::DbErrors::User(e.to_string())))
//     }
// }

// pub async fn get_user_from_uuid(user_uuid: uuid::Uuid, db: &Db) -> Result<crate::db::users::User, BusinessError> {
//     let u = match get_db_user_from_uuid(&db, user_uuid).await {
//         Ok(r) => r,
//         Err(e) => {
//             // User not found, see if we can get the from the beta table
//             // let bu = match get_beta_from_uuid(&db, user_uuid).await {
//             //     Ok(bu) => bu,
//             //     Err(_) => {
//             //         return Err(BusinessError::DbError(DbErrors::User(e.to_string())))
//             //     }
//             // };
//             // crate::db::users::User {
//             //     uuid: bu.uuid,
//             //     username: bu.username,
//             //     email: bu.email,
//             //     mobile: "".to_string(),
//             //     user_state: "".to_string(),
//             //     creation_date: 0,
//             //     verified: false,
//             //     public_key: "".to_string(),
//             // }            
//         },

//     };
//     Ok(u)   
// }