#![cfg(test)]
extern crate std;

use super::*;

use crate::{frontier_nft, FrontierRegistryContract, FrontierRegistryContractClient};
use soroban_sdk::{ symbol_short, vec, Env, IntoVal,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address
};

#[test]
fn test() {
    let env = Env::default();

    // register frontiernft contract 
    let frontier_nft_contract_id = env.register_contract_wasm(None, frontier_nft::WASM);

    // register frontierregistry contract
    let frontier_registry_contract_id = env.register_contract(None, FrontierRegistryContract);
    let frontier_registry_client = FrontierRegistryContractClient::new(&env, &frontier_registry_contract_id);

    // variable
    let admin1: Address = Address::generate(&env);

    let user1: Address = Address::generate(&env);
    let title: String = String::from_str(&env, "The Impact of Social Media on Adolescent Mental Health");
    let description: String = String::from_str(&env, "This essay examines the significant ways technology has transformed contemporary society, exploring its effects on communication, information access, and daily life, while also discussing potential challenges associated with increased reliance on digital tools");
    let uri: String = String::from_str(&env, "0x7B502C3A1F48C8609AE212CDFB639DEE39673F5E");
    let keywords: String = String::from_str(&env, "children");

    // initialize
    frontier_registry_client.initialize(&admin1, &frontier_nft_contract_id);

    // check register
    assert_eq!(
        frontier_registry_client.register(&frontier_nft_contract_id, &user1, &title, &description, &uri, &keywords), 
        Certificate{
            frontier_address: Address::from_string(&String::from_str(&env, "GBOIAU5C66IIH6REQWFMRVEZTBAXXMAMCQBNRIU7V7JHFIRT7XD6DLLN")),
            user_address: user1,
            nft_id: 0,
            title,
            description,
            uri,
            keywords
        }
    );
}

#[test]
fn test_double_register() {
    let env = Env::default();

    // register frontiernft contract 
    let frontier_nft_contract_id = env.register_contract_wasm(None, frontier_nft::WASM);

    // register frontierregistry contract
    let frontier_registry_contract_id = env.register_contract(None, FrontierRegistryContract);
    let frontier_registry_client = FrontierRegistryContractClient::new(&env, &frontier_registry_contract_id);

    // variable
    let admin1: Address = Address::generate(&env);

    let user1: Address = Address::generate(&env);
    let title: String = String::from_str(&env, "The Impact of Social Media on Adolescent Mental Health");
    let description: String = String::from_str(&env, "This essay examines the significant ways technology has transformed contemporary society, exploring its effects on communication, information access, and daily life, while also discussing potential challenges associated with increased reliance on digital tools");
    let uri: String = String::from_str(&env, "0x7B502C3A1F48C8609AE212CDFB639DEE39673F5E");
    let keywords: String = String::from_str(&env, "children");

    // initialize
    frontier_registry_client.initialize(&admin1, &frontier_nft_contract_id);

    frontier_registry_client.register(&frontier_nft_contract_id, &user1, &title, &description, &uri, &keywords);
    let res1 = frontier_registry_client.try_register(&frontier_nft_contract_id, &user1, &title, &description, &uri, &keywords);
    // panic
    assert_eq!(
        res1,
        Err(Ok(Error::AlreadyTitleExist.into()))
    );
    let res2 = frontier_registry_client.try_register(&frontier_nft_contract_id, &user1, &String::from_str(&env, "title"), &description, &uri, &keywords);
    // panic
    assert_eq!(
        res2,
        Err(Ok(Error::AlreadyDescriptionExist.into()))
    );
    
}
