use alloy_chains::NamedChain;
use claim::handle_claim_event;
use database::sea_orm::DatabaseConnection;
use deposit::handle_deposit_event;
use distribute::handle_distribute_event;
use rebalance::handle_rebalance_event;
use shared::AppResult;
use withdraw::handle_withdraw_event;

use crate::ExpectedLog;

mod claim;
mod deposit;
mod distribute;
mod rebalance;
mod withdraw;

pub enum Context {
    Stream,
    Scanner,
}

pub async fn save_log(
    db: &DatabaseConnection,
    chain: NamedChain,
    log: ExpectedLog,
    context: Context,
) -> AppResult<()> {
    match log {
        ExpectedLog::DepositFund(log) => handle_deposit_event(db, chain, log, context).await,
        ExpectedLog::DistributeUserFund(log) => {
            handle_distribute_event(db, chain, log, context).await
        }
        ExpectedLog::RebalanceFundSameChain(log) => {
            handle_rebalance_event(db, chain, log, context).await
        }
        ExpectedLog::WithDrawFundSameChain(log) => {
            handle_withdraw_event(db, chain, log, context).await
        }
        ExpectedLog::Claim(log) => handle_claim_event(db, chain, log, context).await,
        ExpectedLog::DistributeFundCrossChain(log) => Ok(()),
        ExpectedLog::ExecuteReceiveFundCrossChainFailed(log) => Ok(()),
        ExpectedLog::TransferFundCrossChain(log) => Ok(()),
        ExpectedLog::TransferFundFromRouterToFundVaultCrossChain(log) => Ok(()),
        ExpectedLog::WithdrawRequest(log) => Ok(()),
        ExpectedLog::WithdrawFundCrossChainFromOperator(log) => Ok(()),
    }
}
