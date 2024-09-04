use soroban_sdk::{ Env, String };

pub trait FrontierNftMetadata {
    fn name(env: Env) -> String;
    fn description(env: Env) -> String;
    fn uri(env: Env, token_id: u32) -> String;
}
