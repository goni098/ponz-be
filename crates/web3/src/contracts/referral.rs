use alloy::sol;

use crate::client::PublicClient;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    Refferal,
    "src/abis/referral.abi.json"
);

pub type RefferalContract = Refferal::RefferalInstance<PublicClient>;
