use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, sea_query::OnConflict};
use web3::contracts::referral::Refferal::Claim;

use crate::{entities::claim_event, utils::to_decimal};

pub async fn upsert(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    tx_hash: TxHash,
    log_index: u64,
    event: Claim,
) -> Result<(), DbErr> {
    let Claim {
        amount,
        from,
        to,
        claimAt,
    } = event;

    let emit_at = DateTime::from_timestamp(claimAt.to::<i64>(), 0)
        .ok_or(DbErr::Custom("Invalid Claim claimAt timestamp".into()))?
        .into();

    let model = claim_event::ActiveModel {
        chain_id: Set(chain as i64),
        id: Default::default(),
        emit_at: Set(emit_at),
        amount: Set(to_decimal(amount)?),
        from: Set(from.to_string()),
        to: Set(to.to_string()),
        log_index: Set(log_index as i64),
        tx_hash: Set(tx_hash.to_string()),
    };

    claim_event::Entity::insert(model)
        .on_conflict(
            OnConflict::columns([claim_event::Column::TxHash, claim_event::Column::LogIndex])
                .update_columns([
                    claim_event::Column::Amount,
                    claim_event::Column::From,
                    claim_event::Column::To,
                    claim_event::Column::EmitAt,
                ])
                .to_owned(),
        )
        .exec(db_tx)
        .await?;

    Ok(())
}
