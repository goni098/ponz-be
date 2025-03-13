use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

use crate::entities::user;

pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find_by_id(id).one(db).await
}
