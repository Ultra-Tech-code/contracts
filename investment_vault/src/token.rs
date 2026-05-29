use soroban_sdk::{Address, Env, String};
use crate::types::VaultKey;

pub fn balance(env: &Env, account: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&VaultKey::Balance(account.clone()))
        .unwrap_or(0)
}

pub fn mint(env: &Env, to: &Address, amount: i128) {
    let bal = balance(env, to);
    env.storage()
        .persistent()
        .set(&VaultKey::Balance(to.clone()), &(bal + amount));
    let total: i128 = env
        .storage()
        .persistent()
        .get(&VaultKey::TotalShares)
        .unwrap_or(0);
    env.storage()
        .persistent()
        .set(&VaultKey::TotalShares, &(total + amount));
}

pub fn burn(env: &Env, from: &Address, amount: i128) {
    let bal = balance(env, from);
    if bal < amount {
        panic!("insufficient HBS balance");
    }
    env.storage()
        .persistent()
        .set(&VaultKey::Balance(from.clone()), &(bal - amount));
    let total: i128 = env
        .storage()
        .persistent()
        .get(&VaultKey::TotalShares)
        .unwrap_or(0);
    env.storage()
        .persistent()
        .set(&VaultKey::TotalShares, &(total - amount));
}

pub fn transfer(env: &Env, from: &Address, to: &Address, amount: i128) {
    let from_bal = balance(env, from);
    if from_bal < amount {
        panic!("insufficient HBS balance");
    }
    env.storage()
        .persistent()
        .set(&VaultKey::Balance(from.clone()), &(from_bal - amount));
    let to_bal = balance(env, to);
    env.storage()
        .persistent()
        .set(&VaultKey::Balance(to.clone()), &(to_bal + amount));
}

pub fn allowance(env: &Env, from: &Address, spender: &Address) -> i128 {
    let expiry: u32 = env
        .storage()
        .temporary()
        .get(&VaultKey::AllowanceExpiry(from.clone(), spender.clone()))
        .unwrap_or(0);
    if expiry < env.ledger().sequence() {
        return 0;
    }
    env.storage()
        .temporary()
        .get(&VaultKey::Allowance(from.clone(), spender.clone()))
        .unwrap_or(0)
}

pub fn approve(env: &Env, from: &Address, spender: &Address, amount: i128, expiration_ledger: u32) {
    if env.ledger().sequence() > expiration_ledger {
        panic!("expiration in the past");
    }
    env.storage()
        .temporary()
        .set(&VaultKey::Allowance(from.clone(), spender.clone()), &amount);
    env.storage()
        .temporary()
        .set(&VaultKey::AllowanceExpiry(from.clone(), spender.clone()), &expiration_ledger);
}

pub fn transfer_from(env: &Env, spender: &Address, from: &Address, to: &Address, amount: i128) {
    let allowed = allowance(env, from, spender);
    if allowed < amount {
        panic!("insufficient allowance");
    }
    let new_allowance = allowed - amount;
    let expiry: u32 = env
        .storage()
        .temporary()
        .get(&VaultKey::AllowanceExpiry(from.clone(), spender.clone()))
        .unwrap_or(0);
    approve(env, from, spender, new_allowance, expiry);
    transfer(env, from, to, amount);
}

pub fn total_shares(env: &Env) -> i128 {
    env.storage()
        .persistent()
        .get(&VaultKey::TotalShares)
        .unwrap_or(0)
}

pub fn name(_env: &Env) -> String {
    soroban_sdk::String::from_str(_env, "Heliobond Shares")
}

pub fn symbol(_env: &Env) -> String {
    soroban_sdk::String::from_str(_env, "HBS")
}

pub fn decimals() -> u32 {
    7
}
