use rocket::response::status;
use rocket::serde::json::Json; 
use shared::json::user::{User, UpdateUser, ModifiedSecret}; 
use shared::json::GenericResponse;

// use crate::auth::{JwtUser, JwtAdmin};
use crate::db::Db;
use crate::engine::users::{get_users_startwith,};

// #[get("/user/me", format="json")]
// pub fn get_info(jwt_user: &JwtUser) -> Result<Json<User>, status::BadRequest<String>> {

//     Ok(Json(jwt_user.to_user()))
// }

// #[post("/user/update", format="json", data="<updated_user>")]
// pub async fn post_update(jwt_user: &JwtUser, db: Db, updated_user: Json<UpdateUser>) -> Result<Json<GenericResponse>, status::BadRequest<String>> {
//     let user = jwt_user.to_user();

//     let result = update_user(user.uuid, &db, updated_user.into_inner());

//     match result.await {
//         Ok(result) => Ok(Json(result)),
//         Err(e) => Err(status::BadRequest(e.to_string())),
//     }
// }

// #[get("/users/<user_start_with>", format="json")]
// pub async fn get_users<'r>(jwt_admin: JwtAdmin<'r>, db: Db, user_start_with: String) -> Result<Json<Vec<User>>, status::BadRequest<String>> {

//     let _user = jwt_admin.jwt_user;

//     let result = get_users_startwith(&db, user_start_with);

//     match result.await {
//         Ok(result) => Ok(Json(result)),
//         Err(e) => Err(status::BadRequest(e.to_string())),
//     }
// }

// #[post("/user/modify", format="json", data="<modified_user>")]
// pub async fn post_modify<'r>(jwt_admin: JwtAdmin<'r>, db: Db, modified_user: Json<User>) -> Result<Json<GenericResponse>, status::BadRequest<String>> {
//     // To get rid of the warning
//     let _a = jwt_admin;
//     let result = modify_user(&db, modified_user.into_inner());

//     match result.await {
//         Ok(result) => Ok(Json(result)),
//         Err(e) => Err(status::BadRequest(e.to_string())),
//     }

// }

// #[post("/user/secret", format="json", data="<modified_secret>")]
// pub async fn post_secret(jwt_user: &JwtUser, db: Db, modified_secret: Json<ModifiedSecret>) -> Result<Json<GenericResponse>, status::BadRequest<String>> {
//     let user = jwt_user.to_user();
//     let result = modify_secret(user.uuid, &db, modified_secret.into_inner());

//     match result.await {
//         Ok(result) => Ok(Json(result)),
//         Err(e) => Err(status::BadRequest(e.to_string())),
//     }
// }