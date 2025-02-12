use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Bucket(#[from] BucketError),

    #[error("Object is already pinned")]
    ObjectAlreadyPinned {},
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum BucketError {
    #[error("Name of bucket could not be empty")]
    EmptyName,

    #[error("Maximum total size exceeded: {0} / {1}")]
    MaxTotalSizeLimitExceeded(Uint128, Uint128),

    #[error("Maximum objects number exceeded: {0} / {1}")]
    MaxObjectsLimitExceeded(Uint128, Uint128),

    #[error("Maximum object size exceeded: {0} / {1}")]
    MaxObjectSizeLimitExceeded(Uint128, Uint128),

    #[error("Maximum object pins number exceeded: {0} / {1}")]
    MaxObjectPinsLimitExceeded(Uint128, Uint128),

    #[error("Object is already stored")]
    ObjectAlreadyStored,
}
