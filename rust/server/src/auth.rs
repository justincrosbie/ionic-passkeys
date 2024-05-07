use shared::json::user::{User, UserState};

#[derive(PartialEq)]
pub enum Role {
    USER,
    ARBITRATOR,
    ADMIN,
}
pub struct JwtUser{
    pub uuid: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub mobile: String,
    pub role: Role,
    pub create_date: i64,
    pub verified: bool,
}

pub struct JwtArbitrator<'r>{pub jwt_user: &'r JwtUser}

pub struct JwtAdmin<'r>{pub jwt_user: &'r JwtUser}

#[derive(Debug)]
pub enum JwtTokenError {
    Missing,
    Invalid,
    NoDb,
    IssuerInvalid,
    NotArbitrator,
    NotAdmin,
}

impl JwtUser {
    pub fn to_user(&self) -> User {
        let state = match self.role {
            Role::ADMIN => UserState::ADMIN,
            Role::ARBITRATOR => UserState::ARBITRATOR,
            Role::USER => UserState::USER,
        };
        User {
            username: self.username.clone(),
            email: self.email.clone(),
            mobile: self.mobile.clone(),
            uuid: self.uuid,
            user_state: state,
            created_at: self.create_date,
            verified: self.verified,
        }
    }
}

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for &'r JwtUser {
//     type Error = JwtTokenError;

//     async fn from_request(request: &'r Request<'_>) -> Outcome<Self, JwtTokenError> {

//         let jwt_from_header = match request.headers().get_one("Authorization") {
//             Some(jwt_token) => jwt_token,
//             None => return Outcome::Error((Status::Forbidden, JwtTokenError::Missing)),
//         };

//         if !jwt_from_header.to_ascii_lowercase().starts_with(&"Bearer ".to_ascii_lowercase()) {
//             return Outcome::Error((Status::Forbidden, JwtTokenError::Invalid));
//         };

//         let token = jwt_from_header.trim_start_matches("Bearer ");
//         let user_uuid = match JwtPayload::get_iss(&token) {
//             Ok(user_uuid) => user_uuid,
//             Err(_) => return Outcome::Error((Status::Forbidden, JwtTokenError::Invalid)),
//         };

//         // Lookup the user via uuid
//         let db = match request.guard::<Db>().await {
//             Outcome::Success(db) => db,
//             _ => return Outcome::Error((Status::Forbidden, JwtTokenError::NoDb)),
//         };

//         let user = match get_db_user_from_uuid(&db, user_uuid).await {
//             Ok(res) => res,
//             Err(_) => return Outcome::Error((Status::Forbidden, JwtTokenError::IssuerInvalid)),
//         };

//         let user_state = match user.user_state.as_str() {
//             "USER" => UserState::USER,
//             "ADMIN" => UserState::ADMIN,
//             "ARBITRATOR" => UserState::ARBITRATOR,
//             "PREPROVISIONED" => UserState::PREPROVISIONED,
//             _ => UserState::USER,
//         };

//         let usr:User = User {
//             username: user.username.clone(),
//             email: user.email.clone(),
//             uuid: user.uuid,
//             mobile: user.mobile.clone(),
//             user_state: user_state,
//             create_date: user.creation_date,
//             verified: user.verified,
//         };
//         _ = JwtPayload::verify(&token.to_string(), &user.public_key, &usr);

//         let role = match usr.user_state {
//             UserState::ARBITRATOR => Role::ARBITRATOR,
//             UserState::ADMIN => Role::ADMIN,
//             _ => Role::USER,
//         };
//         //let jwt_user =;
//         let jwt_user = request.local_cache(|| 
//             JwtUser{
//                 uuid: user.uuid,
//                 username: user.username,
//                 email: user.email,
//                 mobile: user.mobile,
//                 role: role,
//                 create_date: user.creation_date,
//                 verified: user.verified,
//             }
//         );
//         Outcome::Success(jwt_user)

//     }
// }

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for JwtArbitrator<'r> {
//     type Error = JwtTokenError;

//     async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//         let jwt_user = match request.guard::<&JwtUser>().await {
//             Outcome::Success(user) => user,
//             Outcome::Error(f) => return Outcome::Error(f),
//             Outcome::Forward(f) => return Outcome::Forward(f),
//         };

//         if jwt_user.role.eq(&Role::ADMIN) || jwt_user.role.eq(&Role::ARBITRATOR) {
//             Outcome::Success(JwtArbitrator{jwt_user})
//         } else {
//             Outcome::Error((Status::Forbidden, JwtTokenError::NotArbitrator))
//         }
//     }
// }

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for JwtAdmin<'r> {
//     type Error = JwtTokenError;

//     async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//         let jwt_user = match request.guard::<&JwtUser>().await {
//             Outcome::Success(user) => user,
//             Outcome::Error(f) => return Outcome::Error(f),
//             Outcome::Forward(f) => return Outcome::Forward(f),
//         };

//         if jwt_user.role.eq(&Role::ADMIN) {
//             Outcome::Success(JwtAdmin{jwt_user})
//         } else {
//             Outcome::Error((Status::Forbidden, JwtTokenError::NotAdmin))
//         }
//     }
// }
