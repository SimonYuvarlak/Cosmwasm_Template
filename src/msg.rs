// This file contains the messages that can be sent to the contract.
// Necessary imports
use crate::state::State;
use serde::{Deserialize, Serialize};

// This message is sent when the contract is instantiated
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {}

// This message is sent when the contract is executed
// We are storing different possible messages that will invoke different functions in an enum
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {}

// This message is sent when the contract is queried
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetState {},
}

// This is a custom response that we return for the querying the state
// We use custom return types for the queries because we may want to manipulate or transform the data before returning it
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct StateResponse {}
