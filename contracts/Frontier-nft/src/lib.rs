#![no_std]

mod errors;
mod types;
mod events;

use storage::Storage;

pub use errors::*;
pub use types::*;
pub use events::*;

use soroban_sdk::{
    contract, contractimpl, panic_with_error, Address, String, symbol_short, IntoVal, Map, vec, Env, Val, Symbol, Vec
};

#[contract]
pub struct FrontierNft;

#[contractimpl]
impl FrontierNft {
    pub fn initialize(env: Env, admin: Address) {
        if Admin::Admin.has(&env) {
            panic_with_error!(env, Error::AlreadyInit)
        }
        Admin::Admin.set(&env, &admin);
    }
    pub fn mint(env: Env, to: Address, name: String, description: String, ipfs_hash: String) -> u32 {
        Admin::Admin.get::<Address>(&env).unwrap().require_auth();

        let new_token_id = DataKeyEnumerable::CounterId.get::<u32>(&env).unwrap();

        Self::internal_mint(&env, to, new_token_id, name, description, ipfs_hash);

        DataKeyEnumerable::CounterId.set(&env, &(new_token_id + 1u32));

        new_token_id
    }

    fn internal_mint(env: &Env, to: Address, token_id: u32, name: String, description: String, ipfs_hash: String) {
        if !DataKey::TokenOwner(token_id).has(env) {
            DataKey::TokenOwner(token_id).set(env, &to);

            DatakeyMetadata::Name.set(env, &name);
            DatakeyMetadata::Description.set(env, &description);
            DatakeyMetadata::Uri(token_id).set(env, &ipfs_hash);

            let mut owned_token_indices: Vec<u32> =
                DataKeyEnumerable::OwnedTokenIndices.get(env).unwrap();

            let mut token_id_to_index_map: Map<u32, u32> =
                DataKeyEnumerable::TokenIdToIndex.get(env).unwrap();

            let mut owner_token_indices: Vec<u32> =
                DataKeyEnumerable::OwnerOwnedTokenIds(to.clone())
                    .get(env)
                    .unwrap_or_else(|| Vec::new(env));

            let mut owner_token_index: Map<u32, u32> =
                DataKeyEnumerable::OwnerTokenIdToIndex(to.clone())
                    .get(env)
                    .unwrap_or_else(|| Map::new(env));

            token_id_to_index_map.set(token_id, owned_token_indices.len());

            owned_token_indices.push_back(token_id);

            owner_token_index.set(token_id, owner_token_indices.len());
            owner_token_indices.push_back(token_id);

            DataKeyEnumerable::OwnedTokenIndices.set(env, &owned_token_indices);
            DataKeyEnumerable::TokenIdToIndex.set(env, &token_id_to_index_map);
            DataKeyEnumerable::OwnerOwnedTokenIds(to.clone()).set(env, &owner_token_indices);
            DataKeyEnumerable::OwnerTokenIdToIndex(to.clone()).set(env, &owner_token_index);

            DataKey::Balance(to.clone()).set(env, &owner_token_indices.len());
        } else {
            panic_with_error!(env, Error::TokenAlreadyExist)
        }

        let mut v: Vec<Val> = Vec::new(env);
        v.push_back(to.into_val(env));
        v.push_back(token_id.into());
        Event::Mint.publish(env, v);
    }
}

mod test;
