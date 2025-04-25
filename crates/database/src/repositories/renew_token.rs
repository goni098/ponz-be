use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    sea_query::OnConflict,
};

use crate::entities::renew_token;

pub async fn create_overwrite(
    db: &DatabaseConnection,
    user_id: i64,
    token: String,
) -> Result<(), DbErr> {
    renew_token::Entity::insert(renew_token::ActiveModel {
        id: Default::default(),
        token: Set(token),
        user_id: Set(user_id),
    })
    .on_conflict(
        OnConflict::column(renew_token::Column::UserId)
            .update_column(renew_token::Column::Token)
            .to_owned(),
    )
    .exec(db)
    .await?;

    Ok(())
}

pub async fn find_by_user_id(
    db: &DatabaseConnection,
    user_id: i64,
) -> Result<Option<String>, DbErr> {
    let token = renew_token::Entity::find()
        .filter(renew_token::Column::UserId.eq(user_id))
        .one(db)
        .await?
        .map(|record| record.token);

    Ok(token)
}
