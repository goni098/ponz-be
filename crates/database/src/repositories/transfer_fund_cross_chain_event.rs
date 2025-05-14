use alloy::primitives::TxHash;
use alloy_chains::NamedChain;
use chrono::DateTime;
use sea_orm::{ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, sea_query::OnConflict};
use web3::contracts::cross_chain_router::CrossChainRouter::TransferFundCrossChain;

use crate::{entities::transfer_fund_cross_chain_event, utils::to_decimal};

pub async fn upsert(
    db_tx: &DatabaseTransaction,
    chain: NamedChain,
    tx_hash: TxHash,
    log_index: u64,
    event: TransferFundCrossChain,
) -> Result<(), DbErr> {
    let TransferFundCrossChain {
        depositor,
        depositedToken,
        tokenInForBridge,
        tokenOutForBridge,
        crossChainDexSender,
        crossChainDexReceiver,
        strategyDestination,
        actualAmountBridgeCrossChain,
        distributionFee,
        transportMsg,
        transferFundCrossChainAt,
    } = event;

    let emit_at = DateTime::from_timestamp(transferFundCrossChainAt.to::<i64>(), 0)
        .ok_or(DbErr::Custom(
            "Invalid TransferFundCrossChain transferFundCrossChainAt timestamp".into(),
        ))?
        .into();

    let model = transfer_fund_cross_chain_event::ActiveModel {
        id: Default::default(),
        chain_id: Set(chain as i64),
        emit_at: Set(emit_at),
        log_index: Set(log_index as i64),
        tx_hash: Set(tx_hash.to_string()),
        actual_amount_bridge_cross_chain: Set(to_decimal(actualAmountBridgeCrossChain)?),
        cross_chain_dex_receiver: Set(crossChainDexReceiver.to_string()),
        cross_chain_dex_sender: Set(crossChainDexSender.to_string()),
        deposited_roken: Set(depositedToken.to_string()),
        depositor: Set(depositor.to_string()),
        distribution_fee: Set(to_decimal(distributionFee)?),
        strategy_destination: Set(strategyDestination.to_string()),
        token_in_for_bridge: Set(tokenInForBridge.to_string()),
        token_out_for_bridge: Set(tokenOutForBridge.to_string()),
        transport_msg: Set(transportMsg.to_string()),
    };

    transfer_fund_cross_chain_event::Entity::insert(model)
        .on_conflict(
            OnConflict::columns([
                transfer_fund_cross_chain_event::Column::TxHash,
                transfer_fund_cross_chain_event::Column::LogIndex,
            ])
            .update_columns([
                transfer_fund_cross_chain_event::Column::Depositor,
                transfer_fund_cross_chain_event::Column::DepositedRoken,
                transfer_fund_cross_chain_event::Column::TokenInForBridge,
                transfer_fund_cross_chain_event::Column::TokenOutForBridge,
                transfer_fund_cross_chain_event::Column::CrossChainDexSender,
                transfer_fund_cross_chain_event::Column::CrossChainDexReceiver,
                transfer_fund_cross_chain_event::Column::StrategyDestination,
                transfer_fund_cross_chain_event::Column::ActualAmountBridgeCrossChain,
                transfer_fund_cross_chain_event::Column::DistributionFee,
                transfer_fund_cross_chain_event::Column::TransportMsg,
                transfer_fund_cross_chain_event::Column::EmitAt,
            ])
            .to_owned(),
        )
        .exec(db_tx)
        .await?;

    Ok(())
}
