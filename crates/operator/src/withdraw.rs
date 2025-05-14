pub mod merge_asset;
mod withdraw_on_request;

use alloy_chains::NamedChain;
use database::{repositories, sea_orm::DatabaseConnection};
use shared::AppResult;
use web3::contracts::router::Router::WithdrawRequest;
pub use withdraw_on_request::*;

pub async fn process(chain: NamedChain, db: &DatabaseConnection) -> AppResult<()> {
    let unresolved_events = repositories::withdraw_request_event::find_unresolved(db, 1).await?;

    for unresolved_event in unresolved_events {
        let tx_hash = unresolved_event.tx_hash.clone();
        let log_index = unresolved_event.log_index as u64;
        let event = WithdrawRequest::try_from(unresolved_event)?;

        match withdraw_on_request(chain, event).await {
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

    Ok(())
}
