use alloy::primitives::Address;
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::entities::user;

pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find_by_id(id).one(db).await
}

pub async fn find_by_ref_code(
    db: &DatabaseConnection,
    ref_code: &str,
) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find()
        .filter(user::Column::RefCode.eq(ref_code))
        .one(db)
        .await
}

pub async fn is_refferal_user(db: &DatabaseConnection, address: Address) -> Result<bool, DbErr> {
    let is_refferal_user = user::Entity::find()
        .filter(user::Column::Address.eq(address.to_string()))
        .one(db)
        .await?
        .is_some_and(|user| user.ref_by.is_some());

    Ok(is_refferal_user)
}

pub async fn create_if_not_exist(
    db: &DatabaseConnection,
    address: Address,
    ref_code: String,
    ref_by: Option<String>,
) -> Result<user::Model, DbErr> {
    let address = address.to_string();

    let user = user::Entity::find()
        .filter(user::Column::Address.eq(&address))
        .one(db)
        .await?;

    if let Some(user) = user {
        return Ok(user);
    }
    user::Entity::insert(user::ActiveModel {
        address: Set(address),
        id: Default::default(),
        created_at: Default::default(),
        ref_code: Set(ref_code),
        ref_by: Set(ref_by),
    })
    .exec_with_returning(db)
    .await
}
