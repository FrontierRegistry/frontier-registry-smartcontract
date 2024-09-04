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
    
    let token_id = client.mint(&to_address, &String::from_str(&env, "token_name"), &String::from_str(&env, "token_description"), &String::from_str(&env, "ipfs_hash"));
    
    assert_eq!(client.name(), String::from_str(&env, "token_name"), "wrong token name");
    assert_eq!(client.description(), String::from_str(&env, "token_description"), "wrong token description");
    assert_eq!(client.uri(&token_id), String::from_str(&env, "ipfs_hash"), "wrong uri");
}
