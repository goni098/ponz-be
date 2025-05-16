use alloy::sol;

use crate::clients::PublicClient;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    FundVault,
    "src/abis/fund-vault.abi.json"
);

pub type FundVaultContract = FundVault::FundVaultInstance<PublicClient>;
