#![no_std]
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, String, symbol_short, vec, Env, Symbol, Vec};
use soroban_sdk::token::Client;

mod types;
mod errors;

pub mod frontier_nft {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/frontier_nft.wasm"
    );
}

use types::*;
use errors::*;

#[contract]
pub struct FrontierRegistryContract;

#[contractimpl]
impl FrontierRegistryContract {
    pub fn initialize(env: &Env, admin: Address, contract: Address) {
        let frontier_nft_client = frontier_nft::Client::new(&env, &contract);
        
        frontier_nft_client.initialize(&admin);
    }
    pub fn register(env: &Env, contract: Address, to: Address, title: String, description: String, uri: String, keywords: String) -> Certificate {
        // check title and description
        if ResearchData::Title(to).has(env) {
            panic_with_error(env, Error::AlreadyTitleExist);
        }
        if ResearchData::Description(to).has(env) {
            panic_with_error(env, Error::AlreadyDescriptionExist);
        }

        // mint nft based user's input data
        let frontier_nft_client = frontier_nft::Client::new(&env, &contract);

        frontier_nft_client.initialize(&to);
        let token_id = frontier_nft_client.mint(&to, &title, &description, &uri, &keywords);

        // set certificate data to chain with user address
        let mut owner_certificate_data: Vec<Certificate> = 
            DataKey::CertificateData(to.clone())
                .get(env)
                .unwrap_or_else(|| Vec::new(env));
        owner_certificate_data.push_back(
            Certificate {
                frontier_address: contract,
                user_address: to,
                nft_id: token_id,
                title: title,
                description: description,
                uri: uri,
                keywords: keywords
            });

        DataKey::CertificateData(to.clone()).set(env, &owner_certificate_data);

        // publish cetificate
        Certificate {
            frontier_address: contract,
            user_address: to,
            nft_id: token_id,
            title: title,
            description: description,
            uri: uri,
            keywords: keywords
        }
    }
    pub fn get_research_by_user(env: &Env, user_address: Address) {

    }
}

mod test;
