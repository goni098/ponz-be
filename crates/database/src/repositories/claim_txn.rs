use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, sea_query::OnConflict};
use web3::contracts::referral::Refferal::Claim;

use crate::{entities::claim_txn, utils::to_decimal};

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

    let created_at = DateTime::from_timestamp(claimAt.to::<i64>(), 0)
        .ok_or(DbErr::Custom("Invalid claimAt timestamp".into()))?
        .into();

    let txn = claim_txn::ActiveModel {
        chain_id: Set(chain as i64),
        id: Default::default(),
        created_at: Set(created_at),
        amount: Set(to_decimal(amount)?),
        from: Set(from.to_string()),
        to: Set(to.to_string()),
        log_index: Set(log_index as i64),
        tx_hash: Set(tx_hash.to_string()),
    };

    claim_txn::Entity::insert(txn)
        .on_conflict(
            OnConflict::columns([claim_txn::Column::TxHash, claim_txn::Column::LogIndex])
                .update_columns([
                    claim_txn::Column::Amount,
                    claim_txn::Column::From,
                    claim_txn::Column::To,
                    claim_txn::Column::CreatedAt,
                ])
                .to_owned(),
        )
        .exec(db_tx)
        .await?;

    Ok(())
}
