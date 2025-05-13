use alloy_chains::NamedChain;
use claim::handle_claim_event;
use database::sea_orm::DatabaseConnection;
use deposit::handle_deposit_event;
use distribute::handle_distribute_event;
use rebalance::handle_rebalance_event;
use shared::AppResult;
use withdraw::handle_withdraw_event;

use crate::decode_log::ContractLog;

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
    log: ContractLog,
    context: Context,
) -> AppResult<()> {
    match log {
        ContractLog::DepositFund(log) => handle_deposit_event(db, chain, log, context).await,
        ContractLog::DistributeUserFund(log) => {
            handle_distribute_event(db, chain, log, context).await
        }
        ContractLog::RebalanceFundSameChain(log) => {
            handle_rebalance_event(db, chain, log, context).await
        }
        ContractLog::WithDrawFundSameChain(log) => {
            handle_withdraw_event(db, chain, log, context).await
        }
        ContractLog::Claim(log) => handle_claim_event(db, chain, log, context).await,
        ContractLog::DistributeFundCrossChain(log) => Ok(()),
        ContractLog::ExecuteReceiveFundCrossChainFailed(log) => Ok(()),
        ContractLog::TransferFundCrossChain(log) => Ok(()),
        ContractLog::TransferFundFromRouterToFundVaultCrossChain(log) => Ok(()),
        ContractLog::WithdrawRequest(log) => Ok(()),
    }
}
