// Test file for the contract
#[cfg(test)]
use crate::contract::{execute_logic, instantiate_logic, query_logic};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, StateResponse};
use crate::{execute, instantiate, query};
use cosmwasm_std::{
    coins,
    testing::{mock_env, MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR},
    Addr, Empty, Uint128,
};
use cw_multi_test::{App, BankKeeper, Contract, ContractWrapper, Executor};

// This is a helper function to make testing easier
fn get_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}
