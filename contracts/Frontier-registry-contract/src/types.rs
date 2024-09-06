use soroban_sdk::{contracttype, Address, Env, IntoVal, TryFromVal, String, Val};

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct Certificate {
    pub frontier_address: Address,
    pub user_address: Address,
    pub nft_id: u32,
    pub title: String,
    pub description: String,
    pub uri: String,
    pub keywords: String
}

#[contracttype]
pub enum DataKey {
    CertificateData(Address) // persistant
}

impl storage::Storage for DataKey {
    fn get<V: TryFromVal<Env, Val>>(&self, env: &Env) -> Option<V> {
        storage::Persistent::get(env, self)
    }

    fn set<V: IntoVal<Env, Val>>(&self, env: &Env, val: &V) {
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

#[derive(Clone)]
#[contracttype]
pub enum ResearchData {
    Title(Address, String), // instance
    Description(Address, String) // instance
}

impl storage::Storage for ResearchData {
    fn get<V: TryFromVal<Env, Val>>(&self, env: &Env) -> Option<V> {
        storage::Instance::get(env, self)
    }

    fn set<V: IntoVal<Env, Val>>(&self, env: &Env, val: &V) {
        storage::Instance::set(env, self, val)
    }

    fn has(&self, env: &Env) -> bool {
        storage::Instance::has(env, self)
    }

    fn extend(&self, env: &Env, min_ledger_to_live: u32) -> &Self {
        storage::Instance::extend(env, min_ledger_to_live);
        self
    }

    fn remove(&self, env: &Env) {
        storage::Instance::remove(env, self)
    }
}

