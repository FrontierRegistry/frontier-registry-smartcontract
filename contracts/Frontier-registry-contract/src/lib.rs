#![no_std]
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, String, symbol_short, vec, Env, Symbol, Vec};
use soroban_sdk::token::Client;

mod types;
mod errors;

pub mod frontier_nft {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/frontier_nft.wasm"
    );
}

use types::*;
use errors::*;

#[contract]
pub struct FrontierRegistryContract;

#[contractimpl]
impl FrontierRegistryContract {
    pub fn initialize(env: &Env, frontier_nft_address: Address) {
    }
    pub fn register_research(env: &Env, contract: Address, to: Address, title: String, description: String, uri: String) -> u32 {
        let frontier_nft_client = frontier_nft::Client::new(&env, &contract);

        let token_id = frontier_nft_client.mint(&to, &title, &description, &uri);

        token_id
    }
    pub fn get_nfts_by_user(env: &Env, user_address: Address) {

    }
    pub fn get_nft_info(env: &Env, nft_id: u32) {

    }
}

mod test;
