use base64::{Engine, prelude::BASE64_STANDARD};
use borsh::BorshDeserialize;
use shared::Rlt;

use crate::DISCRIMTINATOR;

pub fn parse_log_data<T: BorshDeserialize>(log_data: &str) -> Rlt<T> {
    let borsh_bytes = &BASE64_STANDARD.decode(log_data)?[DISCRIMTINATOR..];
    let log = T::try_from_slice(borsh_bytes)?;
    Ok(log)
}
