use crate::entities::setting;
use alloy_chains::NamedChain;
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set, Value, prelude::Expr,
};

#[derive(Clone, Copy)]
pub enum Setting {
    ScannedBlock(NamedChain),
}

pub async fn find(db: &DatabaseConnection, setting: Setting) -> Result<Option<String>, DbErr> {
    let value = setting::Entity::find_by_id(setting.as_key_col())
        .one(db)
        .await?
        .map(|record| record.value);

    Ok(value)
}

pub async fn set(db: &DatabaseConnection, setting: Setting, val: String) -> Result<(), DbErr> {
    if find(db, setting).await?.is_some() {
        setting::Entity::update_many()
            .filter(setting::Column::Key.eq(setting.as_key_col()))
            .col_expr(
                setting::Column::Value,
                Expr::value(Value::String(Some(val.into()))),
            )
            .exec(db)
            .await?;

        return Ok(());
    }

    let model = setting::ActiveModel {
        key: Set(setting.as_key_col()),
        value: Set(val),
    };

    setting::Entity::insert(model).exec(db).await?;

    Ok(())
}

impl Setting {
    pub fn as_key_col(&self) -> String {
        match self {
            Self::ScannedBlock(chain) => format!("{}_scanned_block", chain),
        }
    }
}
