use alloy::{
    hex::FromHexError,
    primitives::BigIntConversionError,
    providers::PendingTransactionError,
    transports::{RpcError, TransportErrorKind},
};
use sea_orm::error::DbErr;

use std::{
    borrow::Cow,
    num::{ParseFloatError, ParseIntError},
};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    Custom(Cow<'static, str>),

    #[error(transparent)]
    Db(#[from] DbErr),

    #[error("{0}")]
    EnvError(Cow<'static, str>),

    #[error(transparent)]
    Redis(#[from] redis::RedisError),

    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),

    #[error(transparent)]
    Rpc(#[from] RpcError<TransportErrorKind>),

    #[error(transparent)]
    SolTypes(#[from] alloy::sol_types::Error),

    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    #[error(transparent)]
    HttpClient(#[from] reqwest::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Contract(#[from] alloy::contract::Error),

    #[error(transparent)]
    WaitReceiptTx(#[from] PendingTransactionError),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    FromHex(#[from] FromHexError),

    #[error(transparent)]
    ParseFloat(#[from] ParseFloatError),

    #[error(transparent)]
    BigIntConversion(#[from] BigIntConversionError),
}
