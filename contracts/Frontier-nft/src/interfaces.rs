use soroban_sdk::{ Env, String };

pub trait FrontierNftMetadata {
    fn name(env: Env) -> String;
    fn description(env: Env) -> String;
    fn uri(env: Env, token_id: u32) -> String;
}

pub trait FrontierNft {
    fn balance_of(env: Env, owner: Address) -> u32;
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, token_id: u32);
    fn approve(
        env: Env,
        caller: Address,
        operator: Option<Address>,
        token_id: u32,
        expiration_ledger: u32,
    );
    fn set_approval_for_all(
        env: Env,
        caller: Address,
        owner: Address,
        operator: Address,
        approved: bool,
        expiration_ledger: u32,
    );
    fn get_approved(env: Env, token_id: u32) -> Option<Address>;
    fn is_approval_for_all(env: Env, owner: Address, operator: Address) -> bool;
}
