#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Vec
};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Approved(Address),
}

#[contract]
pub struct AccessRegistry;

#[contractimpl]
impl AccessRegistry {

    /// Initialize contract with an admin address
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }

        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Internal helper: get admin
    fn get_admin(env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap()
    }

    /// Approve a user (admin only)
    pub fn approve(env: Env, admin: Address, user: Address) {
        let stored_admin = Self::get_admin(&env);

        admin.require_auth();
        if admin != stored_admin {
            panic!("Unauthorized");
        }

        env.storage()
            .instance()
            .set(&DataKey::Approved(user.clone()), &true);
    }

    /// Revoke a user (admin only)
    pub fn revoke(env: Env, admin: Address, user: Address) {
        let stored_admin = Self::get_admin(&env);

        admin.require_auth();
        if admin != stored_admin {
            panic!("Unauthorized");
        }

        env.storage()
            .instance()
            .remove(&DataKey::Approved(user));
    }

    /// Check if a user is approved
    pub fn is_approved(env: Env, user: Address) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::Approved(user))
            .unwrap_or(false)
    }

    /// Batch check approvals
    pub fn batch_check(env: Env, users: Vec<Address>) -> Vec<bool> {
        let mut results = Vec::new(&env);

        for user in users.iter() {
            let approved = env.storage()
                .instance()
                .get(&DataKey::Approved(user.clone()))
                .unwrap_or(false);

            results.push_back(approved);
        }

        results
    }
}