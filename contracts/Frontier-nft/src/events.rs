use soroban_sdk::{symbol_short, Env, IntoVal, Symbol, Val};

/**
 * Initialization topic
 */
pub const INITIALIZED_TOPIC: Symbol = symbol_short!("INITIALZD");

pub enum Event {
    Mint,
    Transfer,
    Approve,
    ApproveForAll,
}
impl Event {
    pub fn name(&self) -> &'static str {
        match self {
            Event::Mint => stringify!(Mint),
            Event::Transfer => stringify!(Transfer),
            Event::Approve => stringify!(Approve),
            Event::ApproveForAll => stringify!(ApproveForAll),
        }
    }
    pub fn publish<D>(&self, env: &Env, value: D)
    where
        D: IntoVal<Env, Val>,
    {
        env.events().publish((self.name(),), value);
    }
}
