use soroban_sdk::{symbol_short, Address, Env};

pub fn deposit(env: &Env, from: &Address, usdc_amount: i128, shares_minted: i128) {
    env.events().publish(
        (symbol_short!("vault"), symbol_short!("deposit")),
        (from.clone(), usdc_amount, shares_minted),
    );
}

pub fn withdraw(env: &Env, from: &Address, shares_burned: i128, usdc_returned: i128) {
    env.events().publish(
        (symbol_short!("vault"), symbol_short!("withdraw")),
        (from.clone(), shares_burned, usdc_returned),
    );
}

pub fn project_funded(env: &Env, project_id: u32, amount: i128, recipient: &Address) {
    env.events().publish(
        (symbol_short!("vault"), symbol_short!("funded")),
        (project_id, amount, recipient.clone()),
    );
}
