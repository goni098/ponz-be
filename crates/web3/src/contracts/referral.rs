use alloy::sol;

use crate::clients::PublicClient;

sol!(
    #![sol(extra_derives(serde::Serialize, serde::Deserialize))]
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    #[derive(Debug)]
    Refferal,
    "src/abis/referral.abi.json"
);

pub type RefferalContract = Refferal::RefferalInstance<PublicClient>;
