use soroban_sdk::{contracttype, Address, Env, IntoVal, TryFromVal, String, Val};

#[contracttype]
pub enum DataKey {
    FrontierNftAddress
}