#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String};

mod events;
mod token;
mod types;

pub use types::VaultKey;

#[contract]
pub struct InvestmentVault;

#[contractimpl]
impl InvestmentVault {
    pub fn initialize(env: Env, admin: Address, usdc_sac: Address, registry: Address) {
        if env.storage().instance().has(&VaultKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&VaultKey::Admin, &admin);
        env.storage().instance().set(&VaultKey::UsdcSac, &usdc_sac);
        env.storage().instance().set(&VaultKey::Registry, &registry);
        env.storage().persistent().set(&VaultKey::TotalShares, &0i128);
        env.storage().persistent().set(&VaultKey::TotalInvestments, &0i128);
    }

    // SEP-41 token interface
    pub fn balance(env: Env, account: Address) -> i128 {
        token::balance(&env, &account)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        token::transfer(&env, &from, &to, amount);
    }

    pub fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();
        token::approve(&env, &from, &spender, amount, expiration_ledger);
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        token::allowance(&env, &from, &spender)
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        token::transfer_from(&env, &spender, &from, &to, amount);
    }

    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        token::burn(&env, &from, amount);
    }

    pub fn decimals(_env: Env) -> u32 {
        token::decimals()
    }

    pub fn name(env: Env) -> String {
        token::name(&env)
    }

    pub fn symbol(env: Env) -> String {
        token::symbol(&env)
    }

    pub fn total_supply(env: Env) -> i128 {
        token::total_shares(&env)
    }
}

#[cfg(test)]
mod test;
