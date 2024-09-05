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

    // 
    let admin = Address::generate(&env);
    let to = Address::generate(&env);

    let token_id = client.register(&admin, &to, &String::from_str(&env, "title"), &String::from_str(&env, "description"), &String::from_str(&env, "AJDJAKJDKAJSKDJKS"));


}
