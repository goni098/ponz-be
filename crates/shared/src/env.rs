use std::env::VarError;

use crate::{Rlt, SharedErr};

pub fn read_env(env: &str) -> Rlt<String> {
    std::env::var(env).map_err(|error| match error {
        VarError::NotUnicode(message) => {
            SharedErr::EnvError(message.to_str().unwrap_or_default().to_string().into())
        }
        VarError::NotPresent => {
            SharedErr::EnvError(format!("Missing {} env configuration", env).into())
        }
    })
}
