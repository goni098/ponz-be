use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, sea_query::OnConflict};
use web3::contracts::lz_executor::LzExecutor::TransferFundFromRouterToFundVaultCrossChain;

use crate::{entities::transfer_fund_from_router_to_vault_cross_chain_event, utils::to_decimal};

pub async fn upsert(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    tx_hash: TxHash,
    log_index: u64,
    event: TransferFundFromRouterToFundVaultCrossChain,
) -> Result<(), DbErr> {
    let TransferFundFromRouterToFundVaultCrossChain {
        amount,
        depositedTokenAddress,
        depositor,
        timestamp,
    } = event;

    let emit_at = DateTime::from_timestamp(timestamp.to::<i64>(), 0)
        .ok_or(DbErr::Custom(
            "Invalid TransferFundFromRouterToFundVaultCrossChain timestamp".into(),
        ))?
        .into();

    let model = transfer_fund_from_router_to_vault_cross_chain_event::ActiveModel {
        id: Default::default(),
        chain_id: Set(chain as i64),
        emit_at: Set(emit_at),
        log_index: Set(log_index as i64),
        tx_hash: Set(tx_hash.to_string()),
        amount: Set(to_decimal(amount)?),
        deposited_token_address: Set(depositedTokenAddress.to_string()),
        depositor: Set(depositor.to_string()),
    };

    transfer_fund_from_router_to_vault_cross_chain_event::Entity::insert(model)
        .on_conflict(
            OnConflict::columns([
                transfer_fund_from_router_to_vault_cross_chain_event::Column::TxHash,
                transfer_fund_from_router_to_vault_cross_chain_event::Column::LogIndex,
            ])
            .update_columns([
                transfer_fund_from_router_to_vault_cross_chain_event::Column::Amount,
                transfer_fund_from_router_to_vault_cross_chain_event::Column::DepositedTokenAddress,
                transfer_fund_from_router_to_vault_cross_chain_event::Column::Depositor,
                transfer_fund_from_router_to_vault_cross_chain_event::Column::EmitAt,
            ])
            .to_owned(),
        )
        .exec(db_tx)
        .await?;

    Ok(())
}
