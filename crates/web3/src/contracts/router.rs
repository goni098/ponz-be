use alloy::sol;

use crate::client::PublicClient;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    Router,
    "src/abis/router.abi.json"
);

pub type RouterContract = Router::RouterInstance<PublicClient>;
