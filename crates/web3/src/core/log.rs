use base64::{Engine, prelude::BASE64_STANDARD};
use borsh::BorshDeserialize;
use shared::AppResult;

use crate::DISCRIMINATOR;

pub fn parse_log_data<T: BorshDeserialize>(log_data: &str) -> AppResult<T> {
    let borsh_bytes = &BASE64_STANDARD.decode(log_data)?[DISCRIMINATOR..];
    let log = T::try_from_slice(borsh_bytes)?;
    Ok(log)
}
