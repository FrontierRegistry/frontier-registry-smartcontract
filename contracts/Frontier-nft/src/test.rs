#![cfg(test)]

use super::*;
use soroban_sdk::{
    symbol_short, 
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec,
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

    let res = client.try_initialize(&user1);

    assert_eq!(res, Err(Ok(Error::AlreadyInit.into())));


}
