use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, sea_query::OnConflict};
use web3::contracts::router::Router::DepositFund;

use crate::{entities::deposit_fund_event, enums::TxnStatus, utils::to_decimal};

pub async fn upsert(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    tx_hash: TxHash,
    log_index: u64,
    event: DepositFund,
) -> Result<(), DbErr> {
    let DepositFund {
        receiver,
        tokenAddress,
        depositAmount,
        actualDepositAmount,
        depositedAt,
    } = event;

    let emit_at = DateTime::from_timestamp(depositedAt.to::<i64>(), 0)
        .ok_or(DbErr::Custom(
            "Invalid DepositFund depositedAt timestamp".into(),
        ))?
        .into();

    let model = deposit_fund_event::ActiveModel {
        actual_deposit_amount: Set(to_decimal(actualDepositAmount)?),
        chain_id: Set(chain as i64),
        emit_at: Set(emit_at),
        deposit_amount: Set(to_decimal(depositAmount)?),
        id: Default::default(),
        receiver: Set(receiver.to_string()),
        token_address: Set(tokenAddress.to_string()),
        log_index: Set(log_index as i64),
        tx_hash: Set(tx_hash.to_string()),
        smf_error_msg: Set(None),
        distribute_status: Set(TxnStatus::Pending),
    };

    deposit_fund_event::Entity::insert(model)
        .on_conflict(
            OnConflict::columns([
                deposit_fund_event::Column::TxHash,
                deposit_fund_event::Column::LogIndex,
            ])
            .update_columns([
                deposit_fund_event::Column::Receiver,
                deposit_fund_event::Column::TokenAddress,
                deposit_fund_event::Column::DepositAmount,
                deposit_fund_event::Column::ActualDepositAmount,
                deposit_fund_event::Column::EmitAt,
            ])
            .to_owned(),
        )
        .exec(db_tx)
        .await?;

    Ok(())
}
