use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::entities::pool;

pub async fn find_supported_pools(db: &DatabaseConnection) -> Result<Vec<pool::Model>, DbErr> {
    pool::Entity::find()
        .filter(pool::Column::Enable.eq(true))
        .all(db)
        .await
}
