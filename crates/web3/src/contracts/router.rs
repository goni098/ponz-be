use alloy::sol;

use crate::client::PublicClient;

sol!(
    #![sol(extra_derives(serde::Serialize, serde::Deserialize))]
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    Router,
    "src/abis/router.abi.json"
);

pub type RouterContract = Router::RouterInstance<PublicClient>;
