use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    OverFlowError(#[from] OverflowError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Lockbox expired")]
    LockboxExpired {},
}
