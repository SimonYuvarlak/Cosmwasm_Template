// This is the main file for the contract logic
use crate::msg::InstantiateMsg;
use crate::state::{State, STATE};
use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult, Uint128};

// This function will instantiate the contract when called from the instantiate entry point
pub fn instantiate_logic(
    deps: DepsMut,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let initial_state = State {};
    STATE.save(deps.storage, &initial_state)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

// This is a module for execute functions
// These functions require gas fee
// They are used to alter the state of the contract
pub mod execute_logic {}

// This is a module for query functions
// These functions do not require gas fee
// They are used to read the state of the contract
pub mod query_logic {}
