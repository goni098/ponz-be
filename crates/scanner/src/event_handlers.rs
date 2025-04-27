use alloy::primitives::{Address, TxHash};
use alloy_chains::NamedChain;
use database::sea_orm::DatabaseConnection;
use deposit::handle_deposit_event;
use distribute::handle_distribute_event;
use rebalance::handle_rebalance_event;
use shared::AppResult;
use web3::contracts::router::Router::{
    DepositFund, DistributeUserFund, RebalanceFundSameChain, WithDrawFundSameChain,
};
use withdraw::handle_withdraw_event;

mod claim;
mod deposit;
mod distribute;
mod rebalance;
mod withdraw;

pub enum Event {
    Deposit(DepositFund),
    Distribute(DistributeUserFund),
    Withdraw(WithDrawFundSameChain),
    Rebalance(RebalanceFundSameChain),
}

pub async fn handler(
    db: &DatabaseConnection,
    contract_address: Address,
    tx_hash: TxHash,
    chain: NamedChain,
    event: Event,
) -> AppResult<()> {
    match event {
        Event::Deposit(event) => {
            handle_deposit_event(db, contract_address, tx_hash, chain, event).await
        }
        Event::Distribute(event) => handle_distribute_event(chain, event).await,
        Event::Rebalance(event) => handle_rebalance_event(chain, event).await,
        Event::Withdraw(event) => handle_withdraw_event(chain, event).await,
    }
}
