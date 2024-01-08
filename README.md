# Cosmwasm Smart Contract Template
This repo is an empty template for your cosmwasm contract. 
It has the common libraries and wasm setup.

## Build Rust Code
To build the project you can type `cargo build` to the terminal.

## Build Web Assembly (Wasm)
To build wasm you can type `cargo wasm` to the terminal.

## Check If The Code Is A Valid CosmWasm Contract
To check whether the code you created is a valid cosmwasm contract or not, you can type (in the root of the project folder) `cosmwasm-check ./target/wasm32-unknown-unknown/release/cosmwasm_template.wasm` to the terminal.
I know it look alienish, but it works. You can think this command as a complicated spell taught in Hogwarts.

## Test The Contract
To test the contract, you can type `cargo test` to the terminal.

## Dependencies
Below, you can find the added dependencies to the contract:
- `cosmwasm-schema v1.5.0` for creating schema
- `cosmwasm-std v1.5.0` cosmwasm base library
- `cw-multi-test v0.20.0` library for testing
- `cw-storage-plus v1.2.0` library for state management
- `cw2 v1.1.2` library for providing common utilities for writing smart contracts
- `schemars v0.8.16` library that generates JSON schema documents from Rust types using derive macros
- `serde v1.0.195` library for serialization and deserialization
- `thiserror v1.0.56` library that simplifies the implementation of the standard library's std::error::Error trait for custom error types

- ## Dev Dependencies
- `cw-multi-test v0.20.0`
