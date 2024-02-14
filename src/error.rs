// File: error.rs
// This is the error module.
// It is used to define the error type for the contract.
use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    // Define custom errors here
    #[error("Unauthorized - only {owner} can call this function")]
    NotOwner { owner: String },
}
