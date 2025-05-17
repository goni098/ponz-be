mod components;
mod distribute_when_deposit;
mod distribute_when_rebalance;
mod distribute_when_withdraw_from_operator;

use alloy_chains::NamedChain;
use database::{repositories, sea_orm::DatabaseConnection};
pub use distribute_when_deposit::*;
pub use distribute_when_rebalance::*;
pub use distribute_when_withdraw_from_operator::*;
use pools::ExternalPoolsService;
use shared::AppResult;
use web3::contracts::{
    cross_chain_router::CrossChainRouter::WithdrawFundCrossChainFromOperator,
    router::Router::{DepositFund, RebalanceFundSameChain},
};

pub async fn process_from_db(
    chain: NamedChain,
    db: &DatabaseConnection,
    pools_service: &ExternalPoolsService,
) -> AppResult<()> {
    let unresolved_deposit_fund_events =
        repositories::deposit_fund_event::find_unresolved(db, 1).await?;

    let unresolved_rebalance_fund_same_chain_events =
        repositories::rebalance_fund_same_chain_event::find_unresolved(db, 1).await?;

    let unresolved_withdraw_fund_cross_chain_from_operator_event_events =
        repositories::withdraw_fund_cross_chain_from_operator_event::find_unresolved(db, 1).await?;

    for unresolved_event in unresolved_deposit_fund_events {
        let tx_hash = unresolved_event.tx_hash.clone();
        let log_index = unresolved_event.log_index as u64;
        let event = DepositFund::try_from(unresolved_event)?;

        match distribute_when_deposit(chain, pools_service, event).await {
            Ok(_) => {
                repositories::deposit_fund_event::pin_as_resolved(db, tx_hash, log_index).await?;
            }
            Err(error) => {
                tracing::error!("distribute_when_deposit error {:#?}", error);
                let msg = format!("{:#?}", error);
                repositories::deposit_fund_event::pin_as_failed(db, tx_hash, log_index, msg)
                    .await?;
            }
        }
    }

    for unresolved_event in unresolved_rebalance_fund_same_chain_events {
        let tx_hash = unresolved_event.tx_hash.clone();
        let log_index = unresolved_event.log_index as u64;
        let event = RebalanceFundSameChain::try_from(unresolved_event)?;

        match distribute_when_rebalance(chain, pools_service, event).await {
            Ok(_) => {
                repositories::rebalance_fund_same_chain_event::pin_as_resolved(
                    db, tx_hash, log_index,
                )
                .await?;
            }
            Err(error) => {
                tracing::error!("distribute_when_rebalance error {:#?}", error);
                let msg = format!("{:#?}", error);
                repositories::rebalance_fund_same_chain_event::pin_as_failed(
                    db, tx_hash, log_index, msg,
                )
                .await?;
            }
        }
    }

    for unresolved_event in unresolved_withdraw_fund_cross_chain_from_operator_event_events {
        let tx_hash = unresolved_event.tx_hash.clone();
        let log_index = unresolved_event.log_index as u64;
        let event = WithdrawFundCrossChainFromOperator::try_from(unresolved_event)?;

        match distribute_when_withdraw_from_operator(chain, pools_service, event).await {
            Ok(_) => {
                repositories::withdraw_fund_cross_chain_from_operator_event::pin_as_resolved(
                    db, tx_hash, log_index,
                )
                .await?;
            }
            Err(error) => {
                tracing::error!("distribute_when_withdraw_from_operator error {:#?}", error);
                let msg = format!("{:#?}", error);
                repositories::withdraw_fund_cross_chain_from_operator_event::pin_as_failed(
                    db, tx_hash, log_index, msg,
                )
                .await?;
            }
        }
    }

    Ok(())
}
