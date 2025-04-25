use borsh::BorshDeserialize;
use shared::{AppError, AppResult};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

use crate::DISCRIMINATOR;

pub async fn deserialize_account<T: BorshDeserialize>(
    client: &RpcClient,
    pubkey: &Pubkey,
) -> AppResult<T> {
    let data = client.get_account_data(pubkey).await?;

    let data: &mut &[u8] = &mut &data.as_slice()[DISCRIMINATOR..];

    T::deserialize(data).map_err(|error| {
        if error.to_string().starts_with("AccountNotFound") {
            AppError::SolanaAccountNotFound(*pubkey)
        } else {
            error.into()
        }
    })
}
