use borsh::BorshDeserialize;
use shared::{Rlt, SharedErr};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

use crate::DISCRIMTINATOR;

pub async fn deserialize_account<T: BorshDeserialize>(
    client: &RpcClient,
    pubkey: &Pubkey,
) -> Rlt<T> {
    let data = client.get_account_data(pubkey).await?;

    let data: &mut &[u8] = &mut &data.as_slice()[DISCRIMTINATOR..];

    T::deserialize(data).map_err(|error| {
        if error.to_string().starts_with("AccountNotFound") {
            SharedErr::SolnaAccountNotFound(*pubkey)
        } else {
            error.into()
        }
    })
}
