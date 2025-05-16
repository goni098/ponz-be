pub(crate) mod components;
mod withdraw_from_bridge_when_execute_receive_fund_cross_chain_failed;
mod withdraw_when_request;

use alloy_chains::NamedChain;
use database::{repositories, sea_orm::DatabaseConnection};
use shared::AppResult;
use web3::contracts::{
    router::Router::WithdrawRequest,
    stargate_bridge::StargateBridge::ExecuteReceiveFundCrossChainFailed,
};
pub use withdraw_from_bridge_when_execute_receive_fund_cross_chain_failed::*;
pub use withdraw_when_request::*;

pub async fn process_from_db(chain: NamedChain, db: &DatabaseConnection) -> AppResult<()> {
    let unresolved_withdraw_req_events =
        repositories::withdraw_request_event::find_unresolved(db, 1).await?;
    let unresolved_execute_receive_fund_cross_chain_failed_events =
        repositories::execute_receive_fund_cross_chain_failed_event::find_unresolved(db, 1).await?;

    for unresolved_event in unresolved_withdraw_req_events {
        let tx_hash = unresolved_event.tx_hash.clone();
        let log_index = unresolved_event.log_index as u64;
        let event = WithdrawRequest::try_from(unresolved_event)?;

        match withdraw_when_request(chain, db, event).await {
            Ok(_) => {
                repositories::withdraw_request_event::pin_as_resolved(db, tx_hash, log_index)
                    .await?;
            }
            Err(error) => {
                repositories::withdraw_request_event::pin_as_failed(
                    db,
                    tx_hash,
                    log_index,
                    format!("{:#?}", error),
                )
                .await?;
            }
        }
    }

    for unresolved_event in unresolved_execute_receive_fund_cross_chain_failed_events {
        let tx_hash = unresolved_event.tx_hash.clone();
        let log_index = unresolved_event.log_index as u64;
        let event = ExecuteReceiveFundCrossChainFailed::try_from(unresolved_event)?;

        match withdraw_from_bridge_when_execute_receive_fund_cross_chain_failed(chain, event).await
        {
            Ok(_) => {
                repositories::execute_receive_fund_cross_chain_failed_event::pin_as_resolved(
                    db, tx_hash, log_index,
                )
                .await?;
            }
            Err(error) => {
                repositories::execute_receive_fund_cross_chain_failed_event::pin_as_failed(
                    db,
                    tx_hash,
                    log_index,
                    format!("{:#?}", error),
                )
                .await?;
            }
        }
    }

    Ok(())
}
