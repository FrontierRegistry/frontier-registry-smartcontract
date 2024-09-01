#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, vec, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FrontierRegistryContract);
    let client = FrontierRegistryContractClient::new(&env, &contract_id);


}
