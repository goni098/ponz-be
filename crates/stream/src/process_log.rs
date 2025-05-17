use alloy::{primitives::TxHash, rpc::types::Log};
use alloy_chains::NamedChain;
use database::{repositories, sea_orm::DatabaseConnection};
use operator::withdraw::withdraw_from_bridge_when_execute_receive_fund_cross_chain_failed;
use pools::ExternalPoolsService;
use scanner::handlers::{Context, save_log};
use shared::AppResult;
use web3::logs::{ExpectedLog, decoder::decode_log};

pub async fn process(
    chain: NamedChain,
    db: &DatabaseConnection,
    pools_service: &ExternalPoolsService,
    log: Log,
) -> AppResult<TxHash> {
    let tx_hash = log.transaction_hash.unwrap_or_default();
    let log_index = log.log_index.unwrap_or_default();

    let Some(log) = decode_log(log)? else {
        return Ok(tx_hash);
    };

    save_log(db, chain, log.clone(), Context::Stream).await?;

    match log {
        ExpectedLog::DepositFund(log) => {
            match operator::distribute::distribute_when_deposit(
                chain,
                pools_service,
                log.inner.data,
            )
            .await
            {
                Ok(_) => {
                    repositories::deposit_fund_event::pin_as_resolved(db, tx_hash, log_index)
                        .await?;
                }
                Err(error) => {
                    tracing::error!("distribute_when_deposit error: {:#?}", error);
                    repositories::deposit_fund_event::pin_as_failed(
                        db,
                        tx_hash,
                        log_index,
                        format!("{:#?}", error),
                    )
                    .await?;
                }
            };
        }
        ExpectedLog::RebalanceFundSameChain(log) => {
            match operator::distribute::distribute_when_rebalance(
                chain,
                pools_service,
                log.inner.data,
            )
            .await
            {
                Ok(_) => {
                    repositories::rebalance_fund_same_chain_event::pin_as_resolved(
                        db, tx_hash, log_index,
                    )
                    .await?;
                }
                Err(error) => {
                    tracing::error!("distribute_when_rebalance error: {:#?}", error);
                    let msg = format!("{:#?}", error);
                    repositories::rebalance_fund_same_chain_event::pin_as_failed(
                        db, tx_hash, log_index, msg,
                    )
                    .await?;
                }
            };
        }
        ExpectedLog::WithdrawFundCrossChainFromOperator(log) => {
            match operator::distribute::distribute_when_withdraw_from_operator(
                chain,
                pools_service,
                log.inner.data,
            )
            .await
            {
                Ok(_) => {
                    repositories::withdraw_fund_cross_chain_from_operator_event::pin_as_resolved(
                        db, tx_hash, log_index,
                    )
                    .await?;
                }
                Err(error) => {
                    tracing::error!("distribute_when_withdraw_from_operator error: {:#?}", error);
                    let msg = format!("{:#?}", error);
                    repositories::withdraw_fund_cross_chain_from_operator_event::pin_as_failed(
                        db, tx_hash, log_index, msg,
                    )
                    .await?;
                }
            };
        }
        ExpectedLog::ExecuteReceiveFundCrossChainFailed(log) => {
            match withdraw_from_bridge_when_execute_receive_fund_cross_chain_failed(
                chain,
                log.inner.data,
            )
            .await
            {
                Ok(_) => {
                    repositories::execute_receive_fund_cross_chain_failed_event::pin_as_resolved(
                        db, tx_hash, log_index,
                    )
                    .await?;
                }
                Err(error) => {
                    tracing::error!(
                        "withdraw_from_bridge_when_execute_receive_fund_cross_chain_failed error: {:#?}",
                        error
                    );
                    let msg = format!("{:#?}", error);
                    repositories::execute_receive_fund_cross_chain_failed_event::pin_as_failed(
                        db, tx_hash, log_index, msg,
                    )
                    .await?;
                }
            }
        }
        ExpectedLog::WithdrawRequest(log) => {
            match operator::withdraw::withdraw_when_request(chain, db, log.inner.data).await {
                Ok(_) => {
                    repositories::withdraw_request_event::pin_as_resolved(db, tx_hash, log_index)
                        .await?;
                }
                Err(error) => {
                    tracing::error!("withdraw_when_request error: {:#?}", error);
                    let msg = format!("{:#?}", error);
                    repositories::withdraw_request_event::pin_as_failed(
                        db, tx_hash, log_index, msg,
                    )
                    .await?;
                }
            };
        }
        _ => {}
    };

    Ok(tx_hash)
}
