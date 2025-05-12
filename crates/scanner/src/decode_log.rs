use alloy::{rpc::types::Log, sol_types::SolEvent};
use shared::AppResult;
use web3::contracts::{
    referral::Refferal::Claim,
    router::Router::{
        DepositFund, DistributeUserFund, RebalanceFundSameChain, WithDrawFundSameChain,
    },
};

pub enum ContractEvent {
    DepositFund(Log<DepositFund>),
    DistributeUserFund(Log<DistributeUserFund>),
    WithDrawFundSameChain(Log<WithDrawFundSameChain>),
    RebalanceFundSameChain(Log<RebalanceFundSameChain>),
    Claim(Log<Claim>),
}

pub fn decode_log(log: Log) -> AppResult<Option<ContractEvent>> {
    match log.topic0() {
        Some(&DepositFund::SIGNATURE_HASH) => {
            let log = log.log_decode::<DepositFund>()?;
            Ok(Some(ContractEvent::DepositFund(log)))
        }
        Some(&DistributeUserFund::SIGNATURE_HASH) => {
            let log = log.log_decode::<DistributeUserFund>()?;
            Ok(Some(ContractEvent::DistributeUserFund(log)))
        }
        Some(&WithDrawFundSameChain::SIGNATURE_HASH) => {
            let log = log.log_decode::<WithDrawFundSameChain>()?;
            Ok(Some(ContractEvent::WithDrawFundSameChain(log)))
        }
        Some(&RebalanceFundSameChain::SIGNATURE_HASH) => {
            let log = log.log_decode::<RebalanceFundSameChain>()?;
            Ok(Some(ContractEvent::RebalanceFundSameChain(log)))
        }
        Some(&Claim::SIGNATURE_HASH) => {
            let log = log.log_decode::<Claim>()?;
            Ok(Some(ContractEvent::Claim(log)))
        }
        _ => Ok(None),
    }
}
