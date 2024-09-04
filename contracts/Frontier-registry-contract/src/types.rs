
#[contracttype]
pub enum Amdin {
    Admin,
}

impl storage::Storage for Admin {
    fn get<V: soroban_sdk::TryFromVal<Env, soroban_sdk::Val>>(&self, env: &Env) -> Option<V> {
        storage::Persistent::get(env, self)
    }

    fn set<V: soroban_sdk::IntoVal<Env, soroban_sdk::Val>>(&self, env: &Env, val: &V) {
        storage::Persistent::set(env, self, val)
    }

    fn has(&self, env: &Env) -> bool {
        storage::Persistent::has(env, self)
    }

    fn extend(&self, env: &Env, min_ledger_to_live: u32) -> &Self {
        storage::Persistent::extend(env, self, min_ledger_to_live);
        self
    }

    fn remove(&self, env: &Env) {
        storage::Persistent::remove(env, self)
    }
}

#[contracttype]
pub enum DataKey {
    
}