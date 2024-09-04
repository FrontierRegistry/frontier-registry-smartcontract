#![no_std]
use soroban_sdk::{contract, contractimpl, contractimport, Address, String, symbol_short, vec, Env, Symbol, Vec};

mod types;

pub mod frontier_nft {
    contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/frontier_nft.wasm"
    );
    pub type FrontierNftClient<'a> = Client<'a>;
}

#[contract]
pub struct FrontierRegistryContract;

#[contractimpl]
impl FrontierRegistryContract {
    pub fn initialize(env: &Env, frontier_nft_address: Address) {

    }
}

mod test;
