use rocket::{serde::json::Json, http::Status};
use shared::json::property::Property;

use crate::auth::JwtAdmin;
use crate::db::Db;
use crate::engine::property::*;

#[get("/config/<property>", format="json")]
pub async fn get_config<'r>(property: String, _jwt_adm: JwtAdmin<'r>, db: Db) -> Result<Json<Property>, Status> {

    let prop = get_property(&db, &property).await;
    Ok(Json(prop))
}

#[post("/config", format="json", data="<property>")]
pub async fn post_config<'r>(property: Json<Property>, _jwt_adm: JwtAdmin<'r>, db: Db) -> Result<Json<Property>, Status> {
    let prop = post_property(&db, &property.into_inner()).await;
    Ok(Json(prop))
}
