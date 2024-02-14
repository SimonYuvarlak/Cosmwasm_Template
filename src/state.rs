// Imports
// We import Item from cw_storage_plus which will allow us to store and load data from the state
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

// State of the contract
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct State {}

// This is where we define the variable we want to store in the state
pub const STATE: Item<State> = Item::new("state"); // We use "state" as the key for the variable in the state
