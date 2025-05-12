use alloy::sol;

use crate::{EventArgs, client::PublicClient};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[allow(clippy::too_many_arguments)]
    Refferal,
    "src/abis/referral.abi.json"
);

pub type RefferalContract = Refferal::RefferalInstance<PublicClient>;

impl EventArgs for Refferal::Claim {
    fn json_args(&self) -> serde_json::Value {
        serde_json::json!({
            "amount": self.amount.to_string(),
            "from": self.from.to_string(),
            "to": self.to.to_string(),
            "claimAt:":self.claimAt.to_string()
        })
    }
}
