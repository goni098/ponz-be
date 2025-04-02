use axum::{Json, extract::State};
use database::{repositories, sea_orm::DatabaseConnection};
use serde::Serialize;

use crate::{
    error::{ServerErr, ServerRlt},
    extractors::security::Auth,
};

#[derive(Serialize)]
pub struct Me {
    id: i64,
    address: String,
}

pub async fn handler(
    Auth(claims): Auth,
    State(db): State<DatabaseConnection>,
) -> ServerRlt<Json<Me>> {
    let user = repositories::user::find_by_id(&db, claims.id)
        .await?
        .ok_or(ServerErr::Internal(
            format!("not found user with id {}", claims.id).into(),
        ))?;

    Ok(Json(Me {
        id: user.id,
        address: user.address,
    }))
}
