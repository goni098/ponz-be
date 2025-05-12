use alloy_chains::NamedChain;
use claim::handle_claim_event;
use database::sea_orm::DatabaseConnection;
use deposit::handle_deposit_event;
use distribute::handle_distribute_event;
use rebalance::handle_rebalance_event;
use shared::AppResult;
use withdraw::handle_withdraw_event;

use crate::decode_log::ContractEvent;

mod claim;
mod deposit;
mod distribute;
mod rebalance;
mod withdraw;

pub async fn handler(
    db: &DatabaseConnection,
    chain: NamedChain,
    event: ContractEvent,
    created_at:Dat
) -> AppResult<()> {
    match event {
        ContractEvent::DepositFund(event) => {
            handle_deposit_event(
                db,
                contract_address,
                tx_hash,
                log_index,
                chain,
                event,
                block_timestamp,
            )
            .await
        }
        ContractEvent::DistributeUserFund(event) => {
            handle_distribute_event(
                db,
                contract_address,
                tx_hash,
                log_index,
                chain,
                event,
                block_timestamp,
            )
            .await
        }
        ContractEvent::RebalanceFundSameChain(event) => {
            handle_rebalance_event(
                db,
                contract_address,
                tx_hash,
                log_index,
                chain,
                event,
                block_timestamp,
            )
            .await
        }
        ContractEvent::WithDrawFundSameChain(event) => {
            handle_withdraw_event(
                db,
                contract_address,
                tx_hash,
                log_index,
                chain,
                event,
                block_timestamp,
            )
            .await
        }
        ContractEvent::Claim(event) => {
            handle_claim_event(
                db,
                contract_address,
                tx_hash,
                log_index,
                chain,
                event,
                block_timestamp,
            )
            .await
        }
    }
}
