use alloy_chains::NamedChain;
use database::sea_orm::DatabaseConnection;
use shared::AppResult;

use crate::ExpectedLog;

mod claim;
mod deposit_fund;
mod distribute_fund_cross_chain;
mod distribute_user_fund;
mod execute_receive_fund_cross_chain_failed;
mod rebalance_fund_same_chain;
mod rebalance_fund_same_chain_from_cross_router;
mod transfer_fund_cross_chain;
mod transfer_fund_from_router_to_vault_cross_chain;
mod withdraw_fund_cross_chain_from_operator;
mod withdraw_fund_same_chain;
mod withdraw_request;
pub enum Context {
    Stream,
    Scanner,
}

impl Context {
    pub fn is_scanner(&self) -> bool {
        matches!(self, Self::Scanner)
    }
}

pub async fn save_log(
    db: &DatabaseConnection,
    chain: NamedChain,
    log: ExpectedLog,
    context: Context,
) -> AppResult<()> {
    dbg!(&log);
    match log {
        ExpectedLog::DepositFund(log) => deposit_fund::process(db, chain, log, context).await,
        ExpectedLog::DistributeUserFund(log) => {
            distribute_user_fund::process(db, chain, log, context).await
        }
        ExpectedLog::RebalanceFundSameChain(log) => {
            rebalance_fund_same_chain::process(db, chain, log, context).await
        }
        ExpectedLog::WithDrawFundSameChain(log) => {
            withdraw_fund_same_chain::process(db, chain, log, context).await
        }
        ExpectedLog::Claim(log) => claim::process(db, chain, log, context).await,
        ExpectedLog::DistributeFundCrossChain(log) => {
            distribute_fund_cross_chain::process(db, chain, log, context).await
        }
        ExpectedLog::ExecuteReceiveFundCrossChainFailed(log) => {
            execute_receive_fund_cross_chain_failed::process(db, chain, log, context).await
        }
        ExpectedLog::TransferFundCrossChain(log) => {
            transfer_fund_cross_chain::process(db, chain, log, context).await
        }
        ExpectedLog::TransferFundFromRouterToFundVaultCrossChain(log) => {
            transfer_fund_from_router_to_vault_cross_chain::process(db, chain, log, context).await
        }
        ExpectedLog::WithdrawRequest(log) => {
            withdraw_request::process(db, chain, log, context).await
        }
        ExpectedLog::WithdrawFundCrossChainFromOperator(log) => {
            withdraw_fund_cross_chain_from_operator::process(db, chain, log, context).await
        }
        ExpectedLog::RebalanceFundSameChainFromCrossRouter(log) => {
            rebalance_fund_same_chain_from_cross_router::process(db, chain, log, context).await
        }
    }
}
