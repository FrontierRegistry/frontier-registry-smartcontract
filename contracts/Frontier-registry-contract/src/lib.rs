#![no_std]

mod types;
mod errors;

pub mod frontier_nft {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/frontier_nft.wasm"
    );
}

use storage::Storage;

use types::*;
use errors::*;

use soroban_sdk::{
    contract, contractimpl, panic_with_error, Address, String, symbol_short, IntoVal, Map, vec, Env, Val, Symbol, Vec
};
use soroban_sdk::token::Client;

#[contract]
pub struct FrontierRegistryContract;

#[contractimpl]
impl FrontierRegistryContract {
    pub fn initialize(env: &Env, admin: Address, frontier_nft_contract_id: Address) {
        let frontier_nft_client = frontier_nft::Client::new(&env, &frontier_nft_contract_id);
        
        frontier_nft_client.initialize(&admin);
    }
    // set research data to chain, publish certificate data to user
    pub fn register(env: &Env, frontier_nft_contract_id: Address, to: Address, title: String, description: String, uri: String, keywords: String) -> Certificate {
        // check title and description
        if ResearchData::Title(to.clone()).has(env) {
            panic_with_error!(env, Error::AlreadyTitleExist);
        }
        if ResearchData::Description(to.clone()).has(env) {
            panic_with_error!(env, Error::AlreadyDescriptionExist);
        }

        // mint nft based user's input data
        let frontier_nft_client = frontier_nft::Client::new(&env, &frontier_nft_contract_id);

        let token_id = frontier_nft_client.mint(&to, &title, &description, &uri, &keywords);

        // set certificate data to chain with user address
        let mut owner_certificate_data: Vec<Certificate> = 
            DataKey::CertificateData(to.clone())
                .get(env)
                .unwrap_or_else(|| Vec::new(env));
        let certificate_data = Certificate{
            frontier_address: Address::from_string(&String::from_str(&env, "GBOIAU5C66IIH6REQWFMRVEZTBAXXMAMCQBNRIU7V7JHFIRT7XD6DLLN")).clone(),
            user_address: to.clone(),
            nft_id: token_id.clone(),
            title: title.clone(),
            description: description.clone(),
            uri: uri.clone(),
            keywords: keywords.clone()
        };

        owner_certificate_data.push_back(certificate_data.clone());

        DataKey::CertificateData(to).set(env, &owner_certificate_data);

        // return certificate
        certificate_data
    }
    // get research data of user address
    pub fn get_research_by_user(env: &Env, user_address: Address) -> Vec<Certificate> {
        DataKey::CertificateData(user_address.clone()).get(env).unwrap_or_else(|| Vec::new(env))
    }
}

mod test;
