#![cfg(test)]

use super::*;
use soroban_sdk::{
    symbol_short, 
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec,
    String,
    Env
};

use crate::{
    FrontierNft, 
    FrontierNftClient,
    test::{ errors::Error }
};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FrontierNft);
    let client = FrontierNftClient::new(&env, &contract_id);

    let user1 = Address::generate(&env);

    client.initialize(&user1);

    let to_address = Address::generate(&env);
    
    client.mint(&to_address, &String::from_str(&env, "token_name"), &String::from_str(&env, "token_description"), &String::from_str(&env, "ipfs_hash"));
    

}
