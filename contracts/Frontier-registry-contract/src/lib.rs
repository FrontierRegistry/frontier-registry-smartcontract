#![no_std]
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, String, symbol_short, vec, Env, Symbol, Vec};
use soroban_sdk::token::Client;

mod types;
mod errors;

pub mod frontier_nft {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/frontier_nft.wasm"
    );
    pub type FrontierNftClient<'a> = Client<'a>;
}

use types::*;
use errors::*;

#[contract]
pub struct FrontierRegistryContract;

#[contractimpl]
impl FrontierRegistryContract {
    pub fn initialize(env: &Env, frontier_nft_address: Address) {
    }
    pub fn register_research(env: &Env, title: String, description: String, uri: String) {
    }
}

mod test;
