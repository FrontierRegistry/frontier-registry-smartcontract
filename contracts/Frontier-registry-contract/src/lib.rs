#![no_std]
use soroban_sdk::{contract, contractimpl, Address, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct FrontierRegistryContract;

#[contractimpl]
impl FrontierRegistryContract {
    pub fn initialize(env: Env, admin: Address) {

    }
}

mod test;
