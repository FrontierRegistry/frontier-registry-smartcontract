use soroban_sdk::{ Env, String, Address };

pub trait FrontierNftMetadata {
    fn name(env: Env) -> String;
    fn description(env: Env) -> String;
    fn uri(env: Env, token_id: u32) -> String;
}

pub trait FrontierNftTrait {
    fn balance_of(env: Env, owner: Address) -> u32;
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, token_id: u32);
    fn approve(
        env: Env,
        caller: Address,
        operator: Option<Address>,
        token_id: u32,
        expiration_ledger: u32,
    );
}
