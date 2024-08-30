#![no_std]

mod errors;
mod types;

use storage::Storage;

pub use errors::*;
pub use types::*;

use soroban_sdk::{contract, contractimpl, panic_with_error, Address, String, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct FrontierNft;

#[contractimpl]
impl FrontierNft {
    pub fn initialize(env: Env, admin: Address) {
        if Admin::Admin.has(&env) {
            panic_with_error!(env, Error::AlreadyDeployed)
        }
        Admin::Admin.set(&env, &admin);
    }
    pub fn mint(env: Env, to: Address, title: String, description: String, uri: String) {
        Admin::Admin.get::<Address>(&env).unwrap().require_auth();
    }
}

mod test;
