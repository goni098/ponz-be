use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::entities::pool;

pub async fn find_all_supported(db: &DatabaseConnection) -> Result<Vec<pool::Model>, DbErr> {
    pool::Entity::find()
        .filter(pool::Column::Enable.eq(true))
        .all(db)
        .await
}
