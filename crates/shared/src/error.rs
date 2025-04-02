use sea_orm::error::DbErr;
use solana_client::{
    client_error::{ClientError, ClientErrorKind},
    rpc_request::{RpcError as SolanaRpcError, RpcRequest, RpcResponseErrorData},
    rpc_response::{Response, RpcSimulateTransactionResult},
};
use solana_sdk::message::CompileError;
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::ParsePubkeyError;
use solana_sdk::signature::ParseSignatureError;
use solana_sdk::signer::SignerError;
use std::borrow::Cow;

#[derive(Debug, thiserror::Error)]
pub enum SharedErr {
    #[error("{0}")]
    Custom(Cow<'static, str>),

    #[error(transparent)]
    ParsePubkey(#[from] ParsePubkeyError),

    #[error(transparent)]
    SolanaClient(#[from] ClientError),

    #[error(transparent)]
    SolanaMessageCompile(#[from] CompileError),

    #[error(transparent)]
    SolanaSigner(#[from] SignerError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("{0:#?}")]
    SolanaTxSimmulate(Box<Response<RpcSimulateTransactionResult>>),

    #[error(transparent)]
    SolanaProgram(#[from] ProgramError),

    #[error(transparent)]
    Db(#[from] DbErr),

    #[error("{0}")]
    EnvError(Cow<'static, str>),

    #[error(transparent)]
    SolanaParseSignature(#[from] ParseSignatureError),

    #[error(transparent)]
    Redis(#[from] redis::RedisError),
}

impl SharedErr {
    pub fn get_solana_program_logs(&self) -> Option<&Vec<String>> {
        match self {
            Self::SolanaTxSimmulate(response) => response.value.logs.as_ref(),
            Self::SolanaClient(ClientError {
                request: Some(RpcRequest::SendTransaction),
                kind:
                    ClientErrorKind::RpcError(SolanaRpcError::RpcResponseError {
                        code: _,
                        message: _,
                        data: RpcResponseErrorData::SendTransactionPreflightFailure(response),
                    }),
            }) => response.logs.as_ref(),
            _ => None,
        }
    }
}
