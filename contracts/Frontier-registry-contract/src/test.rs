#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{ symbol_short, vec, Env, IntoVal,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address
};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FrontierRegistryContract);
    let client = FrontierRegistryContractClient::new(&env, &contract_id);

}
