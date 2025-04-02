use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::entities::user;

pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find_by_id(id).one(db).await
}

pub async fn create_if_not_exist(db: &DatabaseConnection, address: String) -> Result<i64, DbErr> {
    let user = user::Entity::find()
        .filter(user::Column::Address.eq(&address))
        .one(db)
        .await?;

    if let Some(user) = user {
        return Ok(user.id);
    }

    let insert_result = user::Entity::insert(user::ActiveModel {
        address: Set(address),
        id: Default::default(),
    })
    .exec(db)
    .await?;

    Ok(insert_result.last_insert_id)
}
