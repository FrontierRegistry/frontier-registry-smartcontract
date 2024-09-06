#![no_std]

mod errors;
mod types;
mod events;
mod interfaces;

use storage::Storage;

pub use errors::*;
pub use types::*;
pub use events::*;
pub use interfaces::{FrontierNftMetadata, FrontierNftTrait};

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

        env.storage().instance().extend_ttl(10000, 10000);

        DataKeyEnumerable::CounterId.set(&env, &u32::MIN);
        DataKeyEnumerable::OwnedTokenIndices.set(&env, &Vec::<u32>::new(&env));
        DataKeyEnumerable::TokenIdToIndex.set(&env, &Map::<u32, u32>::new(&env));
    }
    pub fn mint(env: Env, to: Address, name: String, description: String, uri: String, keywords: String) -> u32 {

        let new_token_id = DataKeyEnumerable::CounterId.get::<u32>(&env).unwrap();

        Self::internal_mint(&env, to, new_token_id, name, description, uri, keywords);

        DataKeyEnumerable::CounterId.set(&env, &(new_token_id + 1u32));

        new_token_id
    }

    fn internal_mint(env: &Env, to: Address, token_id: u32, name: String, description: String, uri: String, keywords: String) {
        if !DataKey::TokenOwner(token_id).has(env) {
            DataKey::TokenOwner(token_id).set(env, &to);

            DatakeyMetadata::Name.set(env, &name);
            DatakeyMetadata::Description.set(env, &description);
            DatakeyMetadata::Uri(token_id).set(env, &uri);
            DatakeyMetadata::Keywords.set(env, &keywords);

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

#[contractimpl]
impl FrontierNftMetadata for FrontierNft {
    fn name(env: Env) -> String {
        DatakeyMetadata::Name.get(&env).unwrap()
    }
    fn description(env: Env) -> String {
        DatakeyMetadata::Description.get(&env).unwrap()
    }
    fn uri(env: Env, token_id: u32) -> String {
        if !DataKey::TokenOwner(token_id).has(&env) {
            panic_with_error!(env, Error::TokenNoExist);
        }

        DatakeyMetadata::Uri(token_id).get(&env).unwrap_or_else(|| String::from_str(&env, "No given token uri"))
    }
    fn keywords(env: Env) -> String {
        DatakeyMetadata::Keywords.get(&env).unwrap()
    }
}

#[contractimpl]
impl FrontierNftTrait for FrontierNft {
    fn balance_of(env: Env, owner: Address) -> u32 {
        DataKey::Balance(owner)
            .extend(&env, 1000)
            .get(&env)
            .unwrap_or(0)
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, token_id: u32) {
        spender.require_auth();
        let is_sender_approved = if spender != from {
            let has_approved =
                if let Some(approved) = DataKey::Approved(token_id).get::<Address>(&env) {
                    // Clear the approval on transfer
                    DataKey::Approved(token_id).remove(&env);
                    approved == spender
                } else {
                    false
                };
            if !has_approved {
                DataKey::Operator(from.clone(), spender).has(&env)
            } else {
                true
            }
        } else {
            true
        };
        if !is_sender_approved {
            panic_with_error!(&env, Error::NotAuthorized);
        }

        if let Some(addr) = DataKey::TokenOwner(token_id).get::<Address>(&env) {
            if addr == from {
                if from != to {
                    // vector containing ids of tokens owned by a specific address:
                    let from_owned_token_ids_key =
                        DataKeyEnumerable::OwnerOwnedTokenIds(from.clone());
                    let to_owned_token_ids_key = DataKeyEnumerable::OwnerOwnedTokenIds(to.clone());
                    let mut from_owned_token_ids: Vec<u32> = from_owned_token_ids_key
                        .get(&env)
                        .unwrap_or_else(|| Vec::new(&env));

                    // A map linking token IDs to their indices for a specific address.
                    let from_owner_token_id_to_index_key =
                        DataKeyEnumerable::OwnerTokenIdToIndex(from.clone());
                    let to_owner_token_id_to_index_key =
                        DataKeyEnumerable::OwnerTokenIdToIndex(to.clone());
                    let mut from_owner_token_id_to_index: Map<u32, u32> =
                        from_owner_token_id_to_index_key
                            .get(&env)
                            .unwrap_or_else(|| Map::new(&env));

                    let mut to_index: Vec<u32> = to_owned_token_ids_key
                        .get(&env)
                        .unwrap_or_else(|| Vec::new(&env));
                    let mut to_token: Map<u32, u32> = to_owner_token_id_to_index_key
                        .get(&env)
                        .unwrap_or_else(|| Map::new(&env));

                    // Remove token from index of from address
                    if let Some(index) = from_owner_token_id_to_index.get(token_id) {
                        // index is the index for an especific address in
                        if let Some(pos) = from_owned_token_ids.iter().position(|x| x == index) {
                            let pos_u32: u32 = pos.try_into().unwrap();
                            from_owned_token_ids.remove(pos_u32);
                        }
                        from_owner_token_id_to_index.remove(token_id);
                    }

                    // Remove token from index of to address
                    to_token.set(token_id, to_index.len());
                    to_index.push_back(token_id);

                    // Update from address vec and map
                    from_owned_token_ids_key.set(&env, &from_owned_token_ids);
                    from_owner_token_id_to_index_key.set(&env, &from_owner_token_id_to_index);
                    DataKey::Balance(from.clone()).set(&env, &from_owned_token_ids.len());

                    // Update to address vec and map
                    to_owner_token_id_to_index_key.set(&env, &to_token);
                    to_owned_token_ids_key.set(&env, &to_index);
                    DataKey::Balance(to.clone()).set(&env, &to_index.len());

                    // Emit the transfer event
                    let mut v: Vec<Val> = Vec::new(&env);
                    v.push_back(from.clone().into_val(&env));
                    v.push_back(to.into_val(&env));
                    v.push_back(token_id.into());
                    Event::Transfer.publish(&env, v);
                }
                DataKey::TokenOwner(token_id).set(&env, &to);
            } else {
                panic_with_error!(&env, Error::NotOwner);
            }
        } else {
            panic_with_error!(&env, Error::NotNFT);
        }
    }

    fn approve(env: Env, caller: Address, operator: Option<Address>, token_id: u32, ttl: u32) {
        if let Some(owner) = DataKey::TokenOwner(token_id).get::<Address>(&env) {
            if owner == caller {
                owner.require_auth();
            } else if DataKey::Operator(owner, caller.clone())
                .get::<bool>(&env)
                .unwrap_or(false)
            {
                caller.require_auth();
            }
        } else {
            panic_with_error!(&env, Error::NotNFT);
        }

        if let Some(to_approve) = operator {
            DataKey::Approved(token_id).set(&env, &to_approve);
            DataKey::Approved(token_id).extend(&env, ttl);

            // Emit the Approved event
            let mut v: Vec<Val> = Vec::new(&env);
            v.push_back(
                DataKey::TokenOwner(token_id)
                    .get::<Address>(&env)
                    .unwrap()
                    .into_val(&env),
            );
            v.push_back(to_approve.into_val(&env));
            v.push_back(token_id.into());
            Event::Approve.publish(&env, v);
        } else {
            DataKey::Approved(token_id).remove(&env);
        }
    }

}

mod test;
