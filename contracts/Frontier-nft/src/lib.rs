#![no_std]
use soroban_sdk::{contract, contractimpl, Address, String, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct FrontierNft;

#[contractimpl]
impl FrontierNft {
    pub fn initialize(env: Env, admin: Address, name: String, symbol: String) {
        
    }
}

mod test;
