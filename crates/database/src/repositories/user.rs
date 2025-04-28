use alloy::primitives::Address;
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::entities::user;

pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find_by_id(id).one(db).await
}

pub async fn create_if_not_exist(db: &DatabaseConnection, address: Address) -> Result<i64, DbErr> {
    let address = address.to_string();

    let user = user::Entity::find()
        .filter(user::Column::Address.eq(&address))
        .one(db)
        .await?;

    if let Some(user) = user {
        return Ok(user.id);
    }

    let user_id = user::Entity::insert(user::ActiveModel {
        address: Set(address),
        id: Default::default(),
    })
    .exec(db)
    .await?
    .last_insert_id;

    Ok(user_id)
}
