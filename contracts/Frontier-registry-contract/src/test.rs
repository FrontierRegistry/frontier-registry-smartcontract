#![cfg(test)]
extern crate std;

use crate::{Frontier_nft, FrontierRegistryContract};
use soroban_sdk::{ symbol_short, vec, Env, IntoVal,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address
};

#[test]
fn test() {
    let env = Env::default();

    let frontier_nft_contract_id = env.register_contract_wasm(None, Frontier_nft::WASM);

    let frontier_registry_contract_id = env.register_contract(None, FrontierRegistryContract);
    let client = FrontierRegistryContractClient::new(&env, &frontier_registry_contract_id);


}
