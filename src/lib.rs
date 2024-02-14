// This is the main file for the contract
// It contains the contract's entry points and the contract's logic
// The contract's logic is implemented as a set of functions and can also be separated into multiple files
// The contract's entry points are implemented as functions with the `#[entry_point]` attribute
// The contract's entry points are the main way to interact with the contract
// Deps and Env are the main types used to interact with the blockchain
// Empty: This is a type that represents an empty struct. It's often used when you don't need any custom configuration for your contract.
// Env: This is a type that provides information about the environment in which the contract is running. It includes details like the block height and time.
// MessageInfo: This is a type that represents information about the message that triggered the contract's execution. It includes details like the sender of the message and the amount of funds sent with it.
// Response: This is a type that represents the contract's response to a message. It can include things like a list of messages to be executed next and attributes to be logged.
// StdResult: This is a type alias for Result that uses StdError as the error type. It's a common return type for contract functions.
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

mod contract; // Here the contract module is imported from the contract.rs file. It is private because it is not used in the entry points. We will be using this to implement the contract logic
mod error; // Here the error module is imported from the error.rs file. It is private because it is not used in the entry points. We will be using this to define the error type for the contract
pub mod msg; // Here the msg module is imported from the msg.rs file. It is public because it is used in the entry points. We will be using this to communicate with the contract from outside world
mod state; // Here the state module is imported from the state.rs file. It is private because it is not used in the entry points. Meaning we will be using this to store the state of the contract and interact with it through the contract logic based on the received messsage
mod test; // Here the test module is imported from the test.rs file. It is private because it is not used in the entry points. We will be using this to run tests

// This is the main entry point for the contract
// It is called when the contract is instantiated
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

// This is the main entry point for the contract
// It is called when the contract is executed
#[entry_point]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    Ok(Response::new())
}

// This is the main entry point for the contract
// It is called when the contract is queried
#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: Empty) -> StdResult<Binary> {
    Ok(Binary::default())
}
